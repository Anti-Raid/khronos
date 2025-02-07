pub mod context;
pub mod event;

use mlua::prelude::*;
use std::cell::RefCell;

/// Creates a userdata iterator from a list of pairs
pub fn create_userdata_iterator<const N: usize>(
    lua: &mlua::Lua,
    pairs: [(String, mlua::Value); N],
) -> LuaResult<mlua::Function> {
    let i = RefCell::new(0);
    lua.create_function(move |lua, _: ()| {
        *i.borrow_mut() += 1;
        let i_val = *i.borrow();
        if i_val <= pairs.len() {
            let (k, v) = pairs[i_val - 1].clone();
            Ok((k, v).into_lua_multi(lua)?)
        } else {
            Ok((mlua::Value::Nil).into_lua_multi(lua)?)
        }
    })
}

/// Creates a userdata iterator from a list of fields
pub fn create_userdata_iterator_with_fields<const N: usize>(
    lua: &mlua::Lua,
    ud: LuaAnyUserData,
    fields: [&'static str; N],
) -> LuaResult<mlua::Function> {
    let i = RefCell::new(0);
    lua.create_function(move |lua, _: ()| {
        *i.borrow_mut() += 1;
        let i_val = *i.borrow();
        if i_val <= fields.len() {
            let k = fields[i_val - 1];

            let v = ud.get::<LuaValue>(k.to_string())?;

            Ok((k, v).into_lua_multi(lua)?)
        } else {
            Ok((mlua::Value::Nil).into_lua_multi(lua)?)
        }
    })
}
