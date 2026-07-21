//! Utility functions for handling buffers

use mluau::prelude::*;

/// A simple way to accept blobs or buffers from usercode, and to pass zero-copy buffers back to Luau.
pub struct Blob(pub bytes::Bytes);

impl IntoLua for Blob {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        Ok(LuaValue::Buffer(lua.create_external_buffer(self.0)?))
    }
}

impl FromLua for Blob {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Buffer(b) => {
                if let Some(bytes) = b.downcast_ref::<bytes::Bytes>() {
                    Ok(Blob(bytes.clone()))
                } else {
                    Ok(Blob(b.to_vec().into()))
                }
            },
            LuaValue::String(str) => Ok(Blob(str.as_bytes().to_vec().into())),
            _ => Err(LuaError::FromLuaConversionError {
                from: "buffer | string",
                to: "Blob".to_string(),
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
        _ => return Err(LuaError::FromLuaConversionError {
            from: "non-bytes",
            to: "bytes".to_string(),
            message: Some("Expected a bytes-like".to_string()),
        }),
    }
}