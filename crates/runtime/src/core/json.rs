use mluau::prelude::*;

use crate::{plugins::antiraid::LUA_DESERIALIZE_OPTIONS, primitives::blob::Blob};

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;
    
    module.set("tojsonstring", lua.create_function(|lua, (value, pretty): (LuaValue, Option<bool>)| {
        let serialized: serde_json::Value = lua.from_value_with(value, LUA_DESERIALIZE_OPTIONS)?;
        let json_str = if pretty.unwrap_or(false) {
            serde_json::to_vec_pretty(&serialized).into_lua_err()?
        } else {
            serde_json::to_vec(&serialized).into_lua_err()?
        };
        lua.create_string(json_str)
    })?)?;

    module.set("fromjsonstring", lua.create_function(|lua, json_str: LuaValue| {
        match json_str {
            LuaValue::String(s) => {
                let deserialized: serde_json::Value = serde_json::from_slice(&s.as_bytes()).into_lua_err()?;
                lua.to_value(&deserialized)
            },
            LuaValue::Buffer(buf) => {
                buf.with_bytes(|bytes| {
                    let deserialized: serde_json::Value = serde_json::from_slice(bytes).into_lua_err()?;
                    lua.to_value(&deserialized)
                })
            },
            LuaValue::UserData(ud) => {
                if let Ok(blob) = ud.borrow::<Blob>() {
                    let deserialized: serde_json::Value = serde_json::from_slice(&blob.data).into_lua_err()?;
                    lua.to_value(&deserialized)
                } else {
                    Err(LuaError::FromLuaConversionError {
                        from: "non-string",
                        to: "JSON string".to_string(),
                        message: Some("Expected a string or buffer for JSON deserialization".to_string()),
                    })
                }
            },
            _ => Err(LuaError::FromLuaConversionError {
                from: "non-string",
                to: "JSON string".to_string(),
                message: Some("Expected a string for JSON deserialization".to_string()),
            }),
        }
    })?)?;



    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}