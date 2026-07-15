use std::collections::HashMap;
use std::io::Read;
use base64::Engine;
use bstr::BString;
use bytes::{Buf, BufMut};
use mlua_scheduler::LuaSchedulerAsync;
use mluau::prelude::*;
use bstr::ByteSlice;
use argon2::Argon2;
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use aes_gcm::{Aes256Gcm, Nonce};
use rand::{Rng, RngCore};
use async_compression::{
    tokio::bufread::{
        GzipDecoder, GzipEncoder
    },
};

use crate::primitives::blob::{Blob, BlobTaker, blob_ref, blob_ref_async};

pub struct TarArchive {
    pub entries: HashMap<BString, bytes::Bytes>,
}

impl TarArchive {
    const MAX_ENTRY_SIZE: usize = 4 * 1024 * 1024;

    /// Makes a empty tar archive
    pub fn new() -> Self {
        TarArchive {
            entries: HashMap::new(),
        }
    }

    pub fn from(b: bytes::Bytes) -> LuaResult<Self> {
        let mut entries = HashMap::new();
        let mut archive = tar::Archive::new(b.as_ref());
        let archive_len = b.len();

        for entry in archive.entries()? {
            let entry = entry?;
            let header = entry.header();
            // Convert the path to a byte string
            let path = header.path_bytes();
            let path_bstr = BString::from(path.as_ref());

            // Read the entry data
            let size = header.size().unwrap_or(0) as usize;

            if size > archive_len {
                return Err(LuaError::external("Entry size exceeds total archive size."));
            }

            if size > Self::MAX_ENTRY_SIZE {
                return Err(LuaError::external(format!(
                    "Archive entry '{}' exceeds maximum allowed size.", 
                    path_bstr
                )));
            }

            let mut data = Vec::with_capacity(size as usize);            
            entry.take(size as u64).read_to_end(&mut data)?;

            entries.insert(path_bstr, data.into());
        }

        Ok(TarArchive { entries })
    }

    /// Writes the tar archive to a Blob
    pub fn to_blob(self) -> LuaResult<bytes::Bytes> {
        let buffer = bytes::BytesMut::new();
        let mut bw = buffer.writer();
        {
            let mut tar = tar::Builder::new(&mut bw);
            for (path, blob) in self.entries {
                let mut header = tar::Header::new_gnu();
                header.set_size(blob.len() as u64);
                tar.append_data(
                    &mut header,
                    path.to_path_lossy(),
                    blob.reader(),
                )?;
            }
            tar.finish()?;
        }

        Ok(bw.into_inner().freeze())
    }
}

impl LuaUserData for TarArchive {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Len, |_, this, ()| {
            Ok(this.entries.len())
        });

        methods.add_method_mut("takefile", |lua, this, name: BString| {
            if let Some(blob) = this.entries.remove(&name) {
                let blob = Blob { data: blob }.into_lua(lua)?;
                Ok(blob)
            } else {
                Ok(LuaNil)
            }
        });

        methods.add_method_mut("addfile", |_, this, (name, blob): (BString, BlobTaker)| {
            this.entries.insert(name, blob.0);
            Ok(())
        });

        methods.add_function("toblob", |_, this: LuaAnyUserData| {
            let this = this.take::<Self>()?;
            this.to_blob().map(|data| Blob { data })
        });

        methods.add_method("entries", |lua, this, ()| {
            let mut entries = Vec::with_capacity(this.entries.len());
            for section in this.entries.keys() {
                entries.push(lua.create_string(section)?);   
            }
            Ok(entries)
        });
    }

    #[cfg(feature = "repl")]
    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
    }
}

fn create_aes256_cipher(key: String, salt: &[u8]) -> LuaResult<Aes256Gcm> {
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        match argon2::ParamsBuilder::new()
            .t_cost(1)
            .m_cost(64 * 1024)
            .p_cost(4)
            .output_len(32)
            .build()
        {
            Ok(params) => params,
            Err(e) => return Err(LuaError::external(format!("Failed to create Argon2 parameters: {}", e))),
        },
    );

    let mut hashed_key = vec![0u8; 32];
    argon2
        .hash_password_into(key.as_bytes(), salt, &mut hashed_key)
        .map_err(|e| LuaError::external(format!("Failed to hash password: {e:?}")))?;

    let cipher = Aes256Gcm::new_from_slice(&hashed_key)
    .map_err(|x| LuaError::external(format!("Aes256 cipher fail: {x}")))?;

    Ok(cipher)
}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set("base64encode", lua.create_function(|_, buf: LuaValue| {
        let encoded = blob_ref(&buf, |b| base64::prelude::BASE64_STANDARD.encode(b))?;
        Ok(encoded)
    })?)?;

    module.set("base64decode", lua.create_function(|_, str: LuaString| {
        let decoded = base64::prelude::BASE64_STANDARD.decode(str.as_bytes())
            .map_err(|e| LuaError::external(format!("Failed to decode base64: {e:?}")))?;
        Ok(Blob { data: decoded.into() })
    })?)?;

    module.set("TarArchive", lua.create_function(|_, blob: Option<BlobTaker>| {
        if let Some(blob) = blob {
            TarArchive::from(blob.0).map_err(LuaError::external)
        } else {
            Ok(TarArchive::new())
        }
    })?)?;

    module.set("aes256encrypt", lua.create_function(|_, (blob, key): (LuaValue, String)| {
        let mut salt = [0u8; 8];
        rand::rng().fill_bytes(&mut salt);

        let cipher = create_aes256_cipher(key, &salt)?;

        let random_slice = rand::rng().random::<[u8; 12]>();
        let nonce = Nonce::from_slice(&random_slice);

        let mut encrypted = blob_ref(&blob, |s| {
            cipher
            .encrypt(nonce, s)
            .map_err(|e| LuaError::external(format!("Failed to encrypt: {:?}", e)))
        })??;

        // Format must be <salt><nonce><ciphertext>
        let mut result = Vec::with_capacity(8 + 12 + encrypted.len());
        result.extend_from_slice(&salt);
        result.extend_from_slice(nonce.as_slice());
        result.append(&mut encrypted);

        Ok(Blob {
            data: result.into(),
        })
    })?)?;

    module.set("aes256decrypt", lua.create_function(|_, (blob, key): (LuaValue, String)| {
        blob_ref(&blob, |blob| {
            if blob.len() < 20 {
                return Err(LuaError::external("Blob data is too short to decrypt".to_string()));
            }

            let salt = &blob[..8];
            let nonce = &blob[8..20];
            let ciphertext = &blob[20..]; 

            let cipher = create_aes256_cipher(key, salt)?;

            let nonce = Nonce::from_slice(nonce);

            let result = cipher
                .decrypt(nonce, ciphertext)
                .map_err(|e| LuaError::external(format!("Failed to decrypt: {:?}", e)))?;

            Ok(Blob {
                data: result.into(),
            })
        })?
    })?)?;

    module.set("compressgzip", lua.create_scheduler_async_function(async move |_lua, (blob, level): (LuaValue, Option<i32>)| {
        async fn compress(data: &[u8], level: Option<i32>) -> LuaResult<Blob> {
            let mut output = Vec::new();
            let input = tokio::io::BufReader::new(data.as_ref());
            let compression_quality = match level {
                Some(lvl) => async_compression::Level::Precise(lvl),
                None => async_compression::Level::Best,
            };

            let mut encoder = GzipEncoder::with_quality(input, compression_quality);
            tokio::io::copy(&mut encoder, &mut output).await?;

            Ok(Blob { data: output.into() })
        }

        blob_ref_async(&blob, async |bytes| compress(bytes, level).await).await?
    })?)?;

    module.set("decompressgzip", lua.create_scheduler_async_function(async move |_lua, blob: LuaValue| {
        async fn decompress(data: &[u8]) -> LuaResult<Blob> {
            let mut output = Vec::new();
            let input = tokio::io::BufReader::new(data.as_ref());

            let mut decoder = GzipDecoder::new(input);
            tokio::io::copy(&mut decoder, &mut output).await?;


            Ok(Blob { data: output.into() })
        }

        blob_ref_async(&blob, decompress).await?
    })?)?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}

