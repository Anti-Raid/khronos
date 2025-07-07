use mluau::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

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

    // Used in iterator
    let lua_global_pairs = Rc::new(
        lua.globals()
            .pairs()
            .collect::<LuaResult<Vec<(LuaValue, LuaValue)>>>()?,
    );

    // Provides iteration over first the users globals, then lua.globals()
    //
    // This is done using a Luau script to avoid borrowing issues
    global_mt.set(
        "__iter",
        lua.create_function(move |lua, globals: LuaTable| {
            let global_pairs = globals
                .pairs()
                .collect::<LuaResult<Vec<(LuaValue, LuaValue)>>>()?;

            let lua_global_pairs = lua_global_pairs.clone();

            let i = Cell::new(0);
            let iter = lua.create_function(move |_lua, ()| {
                let curr_i = i.get();

                if curr_i < global_pairs.len() {
                    let Some((key, value)) = global_pairs.get(curr_i).cloned() else {
                        return Ok((LuaValue::Nil, LuaValue::Nil));
                    };
                    i.set(curr_i + 1);
                    return Ok((key, value));
                }

                if curr_i < global_pairs.len() + lua_global_pairs.len() {
                    let Some((key, value)) =
                        lua_global_pairs.get(curr_i - global_pairs.len()).cloned()
                    else {
                        return Ok((LuaValue::Nil, LuaValue::Nil));
                    };
                    i.set(curr_i + 1);
                    return Ok((key, value));
                }

                Ok((LuaValue::Nil, LuaValue::Nil))
            })?;

            Ok(iter)
        })?,
    )?;

    global_mt.set(
        "__len",
        lua.create_function(move |lua, globals: LuaTable| {
            let globals_len = globals.raw_len();
            let len = lua.globals().raw_len();
            Ok(globals_len + len)
        })?,
    )?;

    // Block getmetatable
    global_mt.set("__metatable", false)?;

    global_tab.set_metatable(Some(global_mt));

    Ok(global_tab)
}
