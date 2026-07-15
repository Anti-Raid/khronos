pub mod lazy;
pub mod opaque;
pub mod blob;

use mluau::prelude::*;

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