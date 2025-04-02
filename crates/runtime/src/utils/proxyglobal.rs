use mlua::prelude::*;

/// Creates a proxy global table that forwards reads to the global table if the key is in the global table
///
/// The resulting proxied global table includes
pub fn proxy_global(lua: &Lua) -> LuaResult<LuaTable> {
    // Setup the global table using a metatable
    //
    // SAFETY: This works because the global table will not change in the VM
    let global_mt = lua.create_table()?;
    let global_tab = lua.create_table()?;

    // Proxy reads to globals if key is in globals, otherwise to the table
    global_mt.set("__index", lua.globals())?;
    global_tab.set("_G", global_tab.clone())?;

    // Provies writes
    // Forward to _G if key is in globals, otherwise to the table
    let globals_ref = lua.globals();
    global_mt.set(
        "__newindex",
        lua.create_function(
            move |_lua, (tab, key, value): (LuaTable, LuaValue, LuaValue)| {
                let v = globals_ref.get::<LuaValue>(key.clone())?;

                if !v.is_nil() {
                    globals_ref.set(key, value)
                } else {
                    tab.raw_set(key, value)
                }
            },
        )?,
    )?;

    lua.gc_collect()?;

    let lua_func = {
        let mut tries = 0;
        loop {
            match lua.load(
                r#"
                local GLOBAL_TAB = ...
                local function iter(t)
                    local on_iter = 0 -- 0 = users globals, 1 = lua.globals()
                    local curr_key = nil
                    return function()
                        if on_iter == 0 then
                            local k, v = next(t, curr_key)
                            if k ~= nil then
                                curr_key = k
                                return k, v
                            end
                            on_iter = 1
                            curr_key = nil
                        end
                        local k, v = next(GLOBAL_TAB, curr_key)
                        if k ~= nil then
                            curr_key = k
                            return k, v
                        end
                    end
                end
                return iter
            "#,
            )
            .set_name("proxy_global_iter")
            .call::<LuaFunction>((lua.globals(),)) {
                Ok(func) => {
                    break func;
                },
                Err(e) => {
                    if tries > 10 {
                        return Err(e);
                    } else {
                        tries += 1;
                        lua.gc_collect()?;
                        log::error!("Failed to create iterator function: {}", e);
                        continue;
                    }
                }
            }    
        }
    };

    // Provides iteration over first the users globals, then lua.globals()
    //
    // This is done using a Luau script to avoid borrowing issues
    global_mt.set(
        "__iter",
        lua_func,
    )?;

    // Block getmetatable
    global_mt.set("__metatable", false)?;

    global_tab.set_metatable(Some(global_mt));

    Ok(global_tab)
}
