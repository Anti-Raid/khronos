use std::collections::HashMap;

use mluau::prelude::*;
use serde::{Deserialize, Serialize};

use crate::primitives::{blob::Blob, lazy::Lazy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KhronosValue {
    Text(String),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    Boolean(bool),
    Buffer(Vec<u8>),   // Binary data
    Vector((f32, f32, f32)), // Luau vector
    Map(Vec<(KhronosValue, KhronosValue)>),
    List(Vec<KhronosValue>),
    Timestamptz(chrono::DateTime<chrono::Utc>),
    Interval(chrono::Duration),
    TimeZone(chrono_tz::Tz),
    LazyStringMap(HashMap<String, String>), // For lazy string maps
    Null,
}

impl KhronosValue {
    fn from_lua_impl(value: LuaValue, lua: &Lua, depth: usize) -> LuaResult<Self> {
        if depth > 10 {
            return Err(LuaError::FromLuaConversionError {
                from: "any",
                to: "KhronosValue".to_string(),
                message: Some("Recursion limit exceeded".to_string()),
            });
        }

        match value {
            LuaValue::String(s) => Ok(KhronosValue::Text(s.to_string_lossy())),
            LuaValue::Integer(i) => Ok(KhronosValue::Integer(i)),
            LuaValue::Number(f) => Ok(KhronosValue::Float(f)),
            LuaValue::Boolean(b) => Ok(KhronosValue::Boolean(b)),
            LuaValue::Buffer(buf) => {
                let data = buf.to_vec();
                Ok(KhronosValue::Buffer(data))
            }
            LuaValue::Vector(v) => Ok(KhronosValue::Vector((v.x(), v.y(), v.z()))),
            LuaValue::Nil => Ok(KhronosValue::Null),
            LuaValue::Table(table) => {
                if table.raw_len() == 0 {
                    // Check for array metatable
                    if let Some(mt) = table.metatable() {
                        if mt == lua.array_metatable() {
                            // Empty list
                            return Ok(KhronosValue::List(Vec::new()));
                        }
                    }

                    // Map
                    let mut map = Vec::new();
                    for pair in table.pairs::<LuaValue, LuaValue>() {
                        let (k, v) = pair?;
                        let k = KhronosValue::from_lua_impl(k, lua, depth + 1)?;
                        let v = KhronosValue::from_lua_impl(v, lua, depth + 1)?;
                        map.push((k, v));
                    }
                    return Ok(KhronosValue::Map(map));
                }
                // Check if the table is a list
                let mut list = Vec::new();
                for v in table.sequence_values::<LuaValue>() {
                    let v = v?;
                    let v = KhronosValue::from_lua_impl(v, lua, depth + 1)?;
                    list.push(v);
                }

                Ok(KhronosValue::List(list))
            }
            LuaValue::UserData(ud) => {
                if let Ok(dt) = ud.borrow::<crate::core::datetime::DateTime<chrono_tz::Tz>>() {
                    return Ok(KhronosValue::Timestamptz(dt.dt.with_timezone(&chrono::Utc)));
                }
                if let Ok(delta) = ud.borrow::<crate::core::datetime::TimeDelta>() {
                    return Ok(KhronosValue::Interval(delta.timedelta));
                }
                if let Ok(tz) = ud.borrow::<crate::core::datetime::Timezone>() {
                    return Ok(KhronosValue::TimeZone(tz.tz));
                }
                if let Ok(i_64) = ud.borrow::<crate::core::typesext::I64>() {
                    return Ok(KhronosValue::Integer(i_64.0));
                }
                if let Ok(u_64) = ud.borrow::<crate::core::typesext::U64>() {
                    return Ok(KhronosValue::UnsignedInteger(u_64.0));
                }
                if let Ok(mut blob) = ud.borrow_mut::<Blob>() {
                    // Take out the contents of the blob 
                    let data = std::mem::take(&mut blob.data);
                    return Ok(KhronosValue::Buffer(data));
                }
                if let Ok(mut s_map) = ud.borrow_mut::<Lazy<HashMap<String, String>>>() {
                    // Take out the contents of the lazy string map 
                    let data = std::mem::take(&mut s_map.data);
                    return Ok(KhronosValue::LazyStringMap(data));
                }

                Err(LuaError::FromLuaConversionError { from: "userdata", to: "DateTime | TimeDelta | TimeZone".to_string(), message: Some("Invalid UserData type. Only DateTime, TimeDelta and TimeZone is supported at this time".to_string()) })
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: "any",
                to: "KhronosValue".to_string(),
                message: Some("Invalid type".to_string()),
            }),
        }
    }

    fn into_lua_impl(self, lua: &Lua, depth: usize) -> LuaResult<LuaValue> {
        if depth > 10 {
            return Err(LuaError::FromLuaConversionError {
                from: "any",
                to: "KhronosValue".to_string(),
                message: Some("Recursion limit exceeded".to_string()),
            });
        }

        match self {
            KhronosValue::Text(s) => Ok(LuaValue::String(lua.create_string(&s)?)),
            KhronosValue::Integer(i) => {
                // If i is above/below the 52 bit precision limit, use a typesext.I64
                let min_luau_integer = -9007199254740991; // 2^53 - 1
                let max_luau_integer = 9007199254740991; // 2^53 - 1
                if i > max_luau_integer || i < min_luau_integer {
                    crate::core::typesext::I64(i).into_lua(lua)
                } else {
                    Ok(LuaValue::Integer(i))
                }
            }
            KhronosValue::UnsignedInteger(i) => crate::core::typesext::U64(i).into_lua(lua), // An UnsignedInteger can only be created through explicit U64 parse
            KhronosValue::Float(f) => Ok(LuaValue::Number(f)),
            KhronosValue::Boolean(b) => Ok(LuaValue::Boolean(b)),
            KhronosValue::Buffer(buf) => {
                let data = buf;
                let lua_buf = lua.create_buffer(data)?;
                Ok(LuaValue::Buffer(lua_buf))
            }
            KhronosValue::Vector(v) => LuaVector::new(v.0, v.1, v.2).into_lua(lua),
            KhronosValue::Map(j) => {
                let table = lua.create_table()?;
                for (k, v) in j.into_iter() {
                    let k = k.into_lua_impl(lua, depth + 1)?;
                    let v = v.into_lua_impl(lua, depth + 1)?;
                    table.set(k, v)?;
                }
                Ok(LuaValue::Table(table))
            }
            KhronosValue::LazyStringMap(m) => {
                let lazy = Lazy::new(m);
                lazy.into_lua(lua)
            }
            KhronosValue::List(l) => {
                let table = lua.create_table()?;
                for v in l.into_iter() {
                    let v = v.into_lua_impl(lua, depth + 1)?;
                    table.set(table.raw_len() + 1, v)?;
                }
                Ok(LuaValue::Table(table))
            }
            KhronosValue::Timestamptz(dt) => {
                crate::core::datetime::DateTime::<chrono_tz::Tz>::from_utc(dt).into_lua(lua)
            }
            KhronosValue::Interval(i) => crate::core::datetime::TimeDelta::new(i).into_lua(lua),
            KhronosValue::TimeZone(tz) => crate::core::datetime::Timezone::new(tz).into_lua(lua),
            KhronosValue::Null => Ok(LuaValue::Nil),
        }
    }
}

impl FromLua for KhronosValue {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        KhronosValue::from_lua_impl(value, lua, 0)
    }
}

impl IntoLua for KhronosValue {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        KhronosValue::into_lua_impl(self, lua, 0)
    }
}