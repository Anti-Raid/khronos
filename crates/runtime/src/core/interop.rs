use mluau::prelude::*;

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    // Null
    module.set("null", lua.null())?;

    // Array metatable
    module.set("array_metatable", lua.array_metatable())?;

    module.set(
        "memusage",
        lua.create_function(|lua, _: ()| Ok(lua.used_memory()))?,
    )?;

    module.set(
        "memory_limit",
        lua.create_function(|lua, _: ()| Ok(lua.memory_limit()?))?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
