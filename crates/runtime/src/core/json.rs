use mluau::prelude::*;

use crate::primitives::{LUA_DESERIALIZE_OPTIONS, blob::blob_ref};

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
        let deser: serde_json::Value = blob_ref(&json_str, |s| serde_json::from_slice(s).into_lua_err())??;
        lua.to_value(&deser)
    })?)?;



    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}