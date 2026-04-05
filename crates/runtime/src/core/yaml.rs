use mluau::prelude::*;

use crate::{primitives::LUA_DESERIALIZE_OPTIONS, primitives::blob::Blob};

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;
    
    module.set("toyamlstring", lua.create_function(|lua, value: LuaValue| {
        let serialized: serde_json::Value = lua.from_value_with(value, LUA_DESERIALIZE_OPTIONS)?;
        let yaml_str = serde_saphyr::to_string(&serialized).into_lua_err()?;
        
        lua.create_string(yaml_str)
    })?)?;

    module.set("fromyamlstring", lua.create_function(|lua, yaml_str: LuaValue| {
        match yaml_str {
            LuaValue::String(s) => {
                let deserialized: serde_json::Value = serde_saphyr::from_slice(s.as_bytes().as_ref()).into_lua_err()?;
                lua.to_value(&deserialized)
            },
            LuaValue::Buffer(buf) => {
                buf.with_bytes(|bytes| {
                    let deserialized: serde_json::Value = serde_saphyr::from_slice(bytes).into_lua_err()?;
                    lua.to_value(&deserialized)
                })
            },
            LuaValue::UserData(ud) => {
                if let Ok(blob) = ud.borrow::<Blob>() {
                    let deserialized: serde_json::Value = serde_saphyr::from_slice(&blob.data).into_lua_err()?;
                    lua.to_value(&deserialized)
                } else {
                    Err(LuaError::FromLuaConversionError {
                        from: "non-string",
                        to: "YAML string".to_string(),
                        message: Some("Expected a string, buffer, or blob for YAML deserialization".to_string()),
                    })
                }
            },
            _ => Err(LuaError::FromLuaConversionError {
                from: "non-string",
                to: "YAML string".to_string(),
                message: Some("Expected a string for YAML deserialization".to_string()),
            }),
        }
    })?)?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}