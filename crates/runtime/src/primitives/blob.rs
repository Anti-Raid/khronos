//! A Blob is a special structure that is owned by Rust
//! and can be used to e.g. avoid copying between Lua and Rust
//! 
//! This core primitive is not available in WASM contexts.yet
//! 
//! When a Blob is passed into a DataStore/Rust, its contents may be moved
//! to Rust leaving a empty Blob. `clone` can be used to avoid this. When this
//! will happen is undefined
//! 
//! Blob is also a way to encrypt/decrypt data with AES-256-GCM (using Argon2id for key derivation)

use mluau::prelude::*;
use rand::{Rng, RngCore};
use argon2::Argon2;
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use aes_gcm::{Aes256Gcm, Nonce};
use zeroize::Zeroize;

pub struct Blob {
    /// The data of the blob
    pub data: Vec<u8>, 
}

/// A simple way to accept blobs or buffers from usercode
pub struct BlobTaker(pub Vec<u8>);

impl FromLua for BlobTaker {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Buffer(b) => Ok(BlobTaker(b.to_vec())),
            LuaValue::UserData(ud) => {
                let mut ud = ud.borrow_mut::<Blob>()?;
                Ok(BlobTaker(std::mem::take(&mut ud.data)))
            },
            _ => Err(LuaError::FromLuaConversionError {
                from: "Blob | buffer",
                to: "BlobTaker".to_string(),
                message: None
            })
        }
    }
}

impl Blob {
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
        .map_err(LuaError::external)?;

        Ok(cipher)
    }
}

impl LuaUserData for Blob {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Len, |_, this, ()| {
            Ok(this.data.len())
        });

        methods.add_function("tobuffer", |lua, ud: LuaAnyUserData| {
            let blob = ud.take::<Self>()?;

            let memory_limit = lua.memory_limit()?;
            let used_memory = lua.used_memory();
            if memory_limit > used_memory && memory_limit - used_memory < blob.data.len() {
                return Err(LuaError::external(format!(
                    "Blob size {} exceeds available memory ({} bytes / {} total bytes)",
                    blob.data.len(),
                    memory_limit - lua.used_memory(),
                    memory_limit
                )));
            }

            let buffer = lua.create_buffer(blob.data)?;
            Ok(buffer)
        });

        methods.add_method("clone", |_, this, ()| {
            Ok(Blob { data: this.data.clone() })
        });

        methods.add_method_mut("drain", |_, this, ()| {
            std::mem::take(&mut this.data);
            Ok(())
        });

        methods.add_method_mut("zeroize", |_, this, ()| {
            this.data.zeroize();
            std::mem::take(&mut this.data);
            Ok(())
        });

        // AES-256 encryption of the blob
        methods.add_method("aes256encrypt", |_, this, key: String| {
            let mut salt = [0u8; 8];
            rand::rng().fill_bytes(&mut salt);

            let cipher = Blob::create_aes256_cipher(key, &salt)?;

            let random_slice = rand::rng().random::<[u8; 12]>();
            let nonce = Nonce::from_slice(&random_slice);

            let mut encrypted = cipher
                .encrypt(nonce, &*this.data)
                .map_err(|e| LuaError::external(format!("Failed to encrypt: {:?}", e)))?;

            // Format must be <salt><nonce><ciphertext>
            let mut result = Vec::with_capacity(8 + 12 + encrypted.len());
            result.extend_from_slice(&salt);
            result.extend_from_slice(nonce.as_slice());
            result.append(&mut encrypted);

            Ok(Blob {
                data: result,
            })
        });

        // AES-256 decryption of the blob
        methods.add_method("aes256decrypt", |_, this, key: String| {
            if this.data.len() < 20 {
                return Err(LuaError::external("Blob data is too short to decrypt".to_string()));
            }

            let salt = &this.data[..8];
            let nonce = &this.data[8..20];
            let ciphertext = &this.data[20..]; 

            let cipher = Blob::create_aes256_cipher(key, salt)?;

            let nonce = Nonce::from_slice(nonce);

            let result = cipher
                .decrypt(nonce, ciphertext)
                .map_err(|e| LuaError::external(format!("Failed to decrypt: {:?}", e)))?;

            Ok(Blob {
                data: result,
            })
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