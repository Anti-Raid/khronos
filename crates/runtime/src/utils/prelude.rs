use mlua::prelude::*;

/// Sets up the prelude for a Lua environment
pub fn setup_prelude(lua: &Lua, env: LuaTable) -> Result<(), LuaError> {
    // Ensure _G.print and _G.eprint are nil
    lua.globals().set("print", LuaValue::Nil)?;
    lua.globals().set("eprint", LuaValue::Nil)?;

    // Prelude code providing some basic functions directly to the Lua VM
    lua.load(
        r#"
            _G.stdout = {}
            _G.stderr = {}

            -- Override print function with function that appends to stdout table
            -- We do this by executing a lua script
            _G.print = function(...)
                local args = {...}
        
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
            _G.eprint = function(...)
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
        "#,
    )
    .set_name("prelude")
    .set_environment(env)
    .exec()?;

    Ok(())
}
