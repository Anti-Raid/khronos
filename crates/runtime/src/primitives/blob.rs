//! A Blob is a special structure that is owned by Rust
//! and can be used to e.g. avoid copying between Lua and Rust

use mluau::prelude::*;

pub struct Blob {
    /// The data of the blob
    pub data: bytes::Bytes, 
}

/// A simple way to accept blobs or buffers from usercode
pub struct BlobTaker(pub bytes::Bytes);

impl FromLua for BlobTaker {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Buffer(b) => Ok(BlobTaker(b.to_vec().into())),
            LuaValue::UserData(ud) => {
                let ud = ud.borrow::<Blob>()?;
                Ok(BlobTaker(ud.data.clone()))
            },
            LuaValue::String(str) => Ok(BlobTaker(str.as_bytes().to_vec().into())),
            _ => Err(LuaError::FromLuaConversionError {
                from: "Blob | buffer",
                to: "BlobTaker".to_string(),
                message: None
            })
        }
    }
}

pub fn blob_ref<R>(val: &LuaValue, f: impl FnOnce(&[u8]) -> R) -> LuaResult<R> {
    match val {
        LuaValue::String(s) => Ok(f(&*s.as_bytes())),
        LuaValue::Buffer(buf) => {
            Ok(buf.with_bytes(f))
        },
        LuaValue::UserData(ud) => {
            let blob = ud.borrow::<Blob>()?;
            Ok(f(&*blob.data))
        },
        _ => return Err(LuaError::FromLuaConversionError {
            from: "non-bytes",
            to: "bytes".to_string(),
            message: Some("Expected a bytes-like".to_string()),
        }),
    }
}

pub async fn blob_ref_async<R>(val: &LuaValue, f: impl AsyncFnOnce(&[u8]) -> R) -> LuaResult<R> {
    match val {
        LuaValue::String(s) => Ok(f(&*s.as_bytes()).await),
        LuaValue::Buffer(buf) => {
            Ok(buf.with_bytes_async(f).await)
        },
        LuaValue::UserData(ud) => {
            let blob = ud.borrow::<Blob>()?;
            Ok(f(&*blob.data).await)
        },
        _ => return Err(LuaError::FromLuaConversionError {
            from: "non-bytes",
            to: "bytes".to_string(),
            message: Some("Expected a bytes-like".to_string()),
        }),
    }
}

impl LuaUserData for Blob {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Len, |_, this, ()| {
            Ok(this.data.len())
        });

        methods.add_function("tobuffer", |lua, ud: LuaAnyUserData| {
            let blob = ud.take::<Self>()?;

            let buffer = lua.create_buffer(blob.data)?;
            Ok(buffer)
        });

        methods.add_method_mut("drain", |_, this, ()| {
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