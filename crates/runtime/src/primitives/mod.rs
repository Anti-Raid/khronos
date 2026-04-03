pub mod context;
pub mod event;
pub mod lazy;
pub mod opaque;
pub mod blob;

use mluau::prelude::*;
use std::cell::RefCell;

/// Creates a userdata iterator from a list of fields
pub fn create_userdata_iterator_with_fields<const N: usize>(
    lua: &mluau::Lua,
    ud: LuaAnyUserData,
    fields: [&'static str; N],
) -> LuaResult<mluau::Function> {
    let i = RefCell::new(0);
    lua.create_function(move |lua, _: ()| {
        let mut i_val = i.try_borrow_mut().map_err(|_| {
            mluau::Error::external("This iterator does not support concurrent access")
        })?;
        *i_val += 1;
        if *i_val <= fields.len() {
            loop {
                if *i_val > fields.len() {
                    return (mluau::Value::Nil).into_lua_multi(lua);
                }

                let k = fields[*i_val - 1];

                let v = match ud.get::<LuaValue>(k.to_string()) {
                    Ok(v) => v,
                    Err(_) => {
                        *i_val += 1;
                        continue;
                    }
                };

                return (k, v).into_lua_multi(lua);
            }
        } else {
            Ok((mluau::Value::Nil).into_lua_multi(lua)?)
        }
    })
}

/// Creates a userdata iterator from a list of fields
pub fn create_userdata_iterator_with_dyn_fields(
    lua: &mluau::Lua,
    ud: LuaAnyUserData,
    fields: Vec<String>,
) -> LuaResult<mluau::Function> {
    let i = RefCell::new(0);
    lua.create_function(move |lua, _: ()| {
        let mut i_val = i.try_borrow_mut().map_err(|_| {
            mluau::Error::external("This iterator does not support concurrent access")
        })?;
        *i_val += 1;
        if *i_val <= fields.len() {
            loop {
                if *i_val > fields.len() {
                    return (mluau::Value::Nil).into_lua_multi(lua);
                }

                let k = &fields[*i_val - 1];

                let v = match ud.get::<LuaValue>(k.to_string()) {
                    Ok(v) => v,
                    Err(_) => {
                        *i_val += 1;
                        continue;
                    }
                };

                return (k.to_string(), v).into_lua_multi(lua);
            }
        } else {
            Ok((mluau::Value::Nil).into_lua_multi(lua)?)
        }
    })
}

pub const LUA_SERIALIZE_OPTIONS: LuaSerializeOptions = LuaSerializeOptions::new()
    .set_array_metatable(true) // PATCH: Set array metatable to true as AntiRaid needs this anyways
    .serialize_none_to_null(false)
    .serialize_unit_to_null(false);

pub const LUA_DESERIALIZE_OPTIONS: LuaDeserializeOptions = LuaDeserializeOptions::new()
    .sort_keys(true)
    .deny_recursive_tables(false)
    .deny_unsupported_types(true)
    .encode_empty_tables_as_array(true)
    .detect_mixed_tables(true);
