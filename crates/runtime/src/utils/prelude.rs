use mlua::prelude::*;

/// Sets up the prelude for a Lua environment
pub fn setup_prelude(lua: Lua, env: LuaTable) -> Result<(), LuaError> {
    // Prelude code providing some basic functions directly to the Lua VM
    lua.load(
        r#"
            local tab = {stdout = {}, stderr = {}}
            -- Override print function with function that appends to stdout table
            -- We do this by executing a lua script
            _G.print = function(...)
                local args = {...}
        
                if #args == 0 then
                    table.insert(tab.stdout, "nil")
                end
    
                local str = ""
                for i = 1, #args do
                    str = str .. tostring(args[i])
                end
                table.insert(tab.stdout, str)
            end

            -- Override eprint function with function that appends to stderr table
            -- We do this by executing a lua script
            _G.eprint = function(...)
                local args = {...}
        
                if #args == 0 then
                    table.insert(tab.stderr, "nil")
                end
    
                local str = ""
                for i = 1, #args do
                    str = str .. tostring(args[i])
                end
                table.insert(tab.stderr, str)
            end

            -- Expose stdout to _G
            _G.stdout = tab.stdout
            -- Expose stderr to _G
            _G.stderr = tab.stderr
        "#,
    )
    .set_name("prelude")
    .set_environment(env)
    .exec()?;

    Ok(())
}
