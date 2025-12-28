use mluau::prelude::*;

/// Disables potentially harmful functions in the Lua environment
/// 
/// Prerequisites: Lua VM must not be sandboxed yet
pub fn disable_harmful(lua: &Lua) -> Result<(), LuaError> {
    // Ensure _G.print and _G.eprint are nil
    lua.globals().set("print", lua.create_function(|_lua, _: ()| {
        Err::<(), LuaError>(LuaError::external("print() is disabled in this environment"))
    })?)?;
    lua.globals().set("eprint", lua.create_function(|_lua, _: ()| {
        Err::<(), LuaError>(LuaError::external("eprint() is disabled in this environment"))
    })?)?;

    Ok(())
}
