use mlua::prelude::*;

/// Disables potentially harmful functions in the Lua environment
pub fn disable_harmful(lua: &Lua) -> Result<(), LuaError> {
    // Ensure _G.print, _G.eprint and _G.require are nil
    lua.globals().set("print", LuaValue::Nil)?;
    lua.globals().set("eprint", LuaValue::Nil)?;
    lua.globals().set("require", LuaValue::Nil)?;
    Ok(())
}

/// Sets up the prelude for a Lua environment
pub fn setup_prelude(lua: &Lua, env: LuaTable) -> Result<(), LuaError> {
    // Prelude code providing some basic functions directly to the Lua VM
    lua.load(
        r#"
            -- Override print function with function that appends to stdout table
            -- We do this by executing a lua script
            local print = function(...)
                local args = {...}

                if not typeof(args) == "table" then
                    args = {args}
                end
        
                if #args == 0 then
                    table.insert(_G.stdout, "nil")
                end
    
                local str = ""
                for i = 1, #args do
                    str = str .. tostring(args[i])
                end
                table.insert(_G.stdout, str)
            end

            -- Override eprint function with function that appends to stderr table
            -- We do this by executing a lua script
            local eprint = function(...)
                local args = {...}
        
                if #args == 0 then
                    table.insert(_G.stderr, "nil")
                end
    
                local str = ""
                for i = 1, #args do
                    str = str .. tostring(args[i])
                end
                table.insert(_G.stderr, str)
            end

            rawset(_G, "stdout", {})
            rawset(_G, "stderr", {})
            rawset(_G, "print", print)
            rawset(_G, "eprint", eprint)
        "#,
    )
    .set_name("prelude")
    .set_environment(env)
    .exec()
    .map_err(|e| LuaError::external(format!("Failed to load prelude: {}", e)))?;

    Ok(())
}
