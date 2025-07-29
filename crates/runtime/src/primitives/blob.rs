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
            LuaValue::String(str) => Ok(BlobTaker(str.as_bytes().to_vec())),
            _ => Err(LuaError::FromLuaConversionError {
                from: "Blob | buffer",
                to: "BlobTaker".to_string(),
                message: None
            })
        }
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

        methods.add_method_mut("drain", |_, this, ()| {
            std::mem::take(&mut this.data);
            Ok(())
        });

        methods.add_method_mut("zeroize", |_, this, ()| {
            this.data.zeroize();
            std::mem::take(&mut this.data);
            Ok(())
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