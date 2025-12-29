use mluau::prelude::*;
use serde::{Deserialize, Serialize};

use crate::primitives::blob::Blob;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct KhronosBuffer(pub Vec<u8>);

impl KhronosBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        KhronosBuffer(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "KhronosProxy", into = "KhronosProxy")]
pub enum KhronosValue {
    Text(String),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    Boolean(bool),
    Buffer(KhronosBuffer),   // Binary data
    Vector((f32, f32, f32)), // Luau vector
    Map(indexmap::IndexMap<String, KhronosValue>),
    List(Vec<KhronosValue>),
    Timestamptz(chrono::DateTime<chrono::Utc>),
    Interval(chrono::Duration),
    TimeZone(chrono_tz::Tz),
    Null,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "___khronosValType___", content = "value")]
#[serde(rename_all = "lowercase")] 
enum KhronosSpecial {
    Buffer(KhronosBuffer),
    Vector((f32, f32, f32)),
    Timestamptz(chrono::DateTime<chrono::Utc>),
    Interval(chrono::Duration),
    TimeZone(chrono_tz::Tz),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum KhronosProxy {
    // Note that order matters here as serde(untagged) will try each variant in order.

    // First, check special types
    Special(KhronosSpecial),

    // Primitives
    Boolean(bool),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    Text(String),
    List(Vec<KhronosValue>),
    
    // Map (as this can overlap with other types, it must be last)
    Map(indexmap::IndexMap<String, KhronosValue>),
    
    Null,
}

impl From<KhronosProxy> for KhronosValue {
    fn from(proxy: KhronosProxy) -> Self {
        match proxy {
            KhronosProxy::Special(s) => match s {
                KhronosSpecial::Buffer(b) => KhronosValue::Buffer(b),
                KhronosSpecial::Vector(v) => KhronosValue::Vector(v),
                KhronosSpecial::Timestamptz(t) => KhronosValue::Timestamptz(t),
                KhronosSpecial::Interval(i) => KhronosValue::Interval(i),
                KhronosSpecial::TimeZone(t) => KhronosValue::TimeZone(t),
            },
            KhronosProxy::Boolean(b) => KhronosValue::Boolean(b),
            KhronosProxy::Integer(i) => KhronosValue::Integer(i),
            KhronosProxy::UnsignedInteger(u) => KhronosValue::UnsignedInteger(u),
            KhronosProxy::Float(f) => KhronosValue::Float(f),
            KhronosProxy::Text(t) => KhronosValue::Text(t),
            KhronosProxy::List(l) => KhronosValue::List(l),
            KhronosProxy::Map(m) => KhronosValue::Map(m),
            KhronosProxy::Null => KhronosValue::Null,
        }
    }
}

impl From<KhronosValue> for KhronosProxy {
    fn from(val: KhronosValue) -> Self {
        match val {
            KhronosValue::Buffer(b) => KhronosProxy::Special(KhronosSpecial::Buffer(b)),
            KhronosValue::Vector(v) => KhronosProxy::Special(KhronosSpecial::Vector(v)),
            KhronosValue::Timestamptz(t) => KhronosProxy::Special(KhronosSpecial::Timestamptz(t)),
            KhronosValue::Interval(i) => KhronosProxy::Special(KhronosSpecial::Interval(i)),
            KhronosValue::TimeZone(t) => KhronosProxy::Special(KhronosSpecial::TimeZone(t)),
            KhronosValue::Boolean(b) => KhronosProxy::Boolean(b),
            KhronosValue::Integer(i) => KhronosProxy::Integer(i),
            KhronosValue::UnsignedInteger(u) => KhronosProxy::UnsignedInteger(u),
            KhronosValue::Float(f) => KhronosProxy::Float(f),
            KhronosValue::Text(t) => KhronosProxy::Text(t),
            KhronosValue::List(l) => KhronosProxy::List(l),
            KhronosValue::Map(m) => KhronosProxy::Map(m),
            KhronosValue::Null => KhronosProxy::Null,
        }
    }
}

impl FromLua for Box<KhronosValue> {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let value = KhronosValue::from_lua_impl(value, lua, 0)?;
        Ok(Box::new(value))
    }
}

impl IntoLua for Box<KhronosValue> {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        (*self).into_lua_impl(lua, 0)
    }
}

impl KhronosValue {
    pub fn kind(&self) -> &'static str {
        match self {
            KhronosValue::Text(_) => "text",
            KhronosValue::Integer(_) => "integer",
            KhronosValue::UnsignedInteger(_) => "unsigned_integer",
            KhronosValue::Float(_) => "float",
            KhronosValue::Boolean(_) => "boolean",
            KhronosValue::Buffer(_) => "buffer",
            KhronosValue::Vector(_) => "vector",
            KhronosValue::Map(_) => "map",
            KhronosValue::List(_) => "list",
            KhronosValue::Timestamptz(_) => "timestamptz",
            KhronosValue::Interval(_) => "interval",
            KhronosValue::TimeZone(_) => "timezone",
            KhronosValue::Null => "null",
        }
    }

    pub fn from_lua_impl(value: LuaValue, lua: &Lua, depth: usize) -> LuaResult<Self> {
        if depth > 10 {
            return Err(LuaError::FromLuaConversionError {
                from: "any",
                to: "KhronosValue".to_string(),
                message: Some("Recursion limit exceeded".to_string()),
            });
        }

        match value {
            LuaValue::String(s) => Ok(KhronosValue::Text(s.to_string_lossy().to_string())),
            LuaValue::Integer(i) => Ok(KhronosValue::Integer(i)),
            LuaValue::Number(f) => Ok(KhronosValue::Float(f)),
            LuaValue::Boolean(b) => Ok(KhronosValue::Boolean(b)),
            LuaValue::Buffer(buf) => {
                let data = buf.to_vec();
                Ok(KhronosValue::Buffer(KhronosBuffer::new(data)))
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
                    let mut map = indexmap::IndexMap::new();
                    for pair in table.pairs::<String, LuaValue>() {
                        let (k, v) = pair?;
                        let v = KhronosValue::from_lua_impl(v, lua, depth + 1)?;
                        map.insert(k, v);
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
                    return Ok(KhronosValue::Buffer(KhronosBuffer::new(data)));
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

    pub fn into_lua_impl(self, lua: &Lua, depth: usize) -> LuaResult<LuaValue> {
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
                let data = buf.0;
                let lua_buf = lua.create_buffer(data)?;
                Ok(LuaValue::Buffer(lua_buf))
            }
            KhronosValue::Vector(v) => LuaVector::new(v.0, v.1, v.2).into_lua(lua),
            KhronosValue::Map(j) => {
                let table = lua.create_table()?;
                for (k, v) in j.into_iter() {
                    let v = v.into_lua_impl(lua, depth + 1)?;
                    table.set(k, v)?;
                }
                Ok(LuaValue::Table(table))
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