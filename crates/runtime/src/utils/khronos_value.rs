use mluau::prelude::*;
use serde::{Deserialize, Serialize};

use crate::primitives::blob::Blob;

const KHRONOS_VALUE_TYPE_KEY: &str = "___khronosValType___";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct KhronosBuffer(pub Vec<u8>);

impl KhronosBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        KhronosBuffer(data)
    }
}

#[derive(Debug, Clone)]
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

    pub fn into_serde_json_value(
        self,
        depth: usize,
        preserve_types: bool,
    ) -> Result<serde_json::Value, crate::Error> {
        if depth > 10 {
            return Err("Recursion limit exceeded".into());
        }

        Ok(match self {
            KhronosValue::Text(s) => serde_json::Value::String(s),
            KhronosValue::Integer(i) => serde_json::Value::Number(serde_json::Number::from(i)),
            KhronosValue::UnsignedInteger(i) => {
                serde_json::Value::Number(serde_json::Number::from(i))
            }
            KhronosValue::Float(f) => {
                serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap())
            }
            KhronosValue::Boolean(b) => serde_json::Value::Bool(b),
            KhronosValue::Buffer(buf) => {
                if !preserve_types {
                    serde_json::to_value(buf)
                        .map_err(|e| format!("Failed to serialize Buffer: {e}"))?
                } else {
                    serde_json::json!({
                        KHRONOS_VALUE_TYPE_KEY: "buffer",
                        "value": serde_json::to_value(buf)
                        .map_err(|e| format!("Failed to serialize Buffer: {e}"))?
                    })
                }
            }
            KhronosValue::Vector(v) => {
                if !preserve_types {
                    serde_json::to_value(v)
                        .map_err(|e| format!("Failed to serialize Vector: {e}"))?
                } else {
                    serde_json::json!({
                        KHRONOS_VALUE_TYPE_KEY: "vector",
                        "value": serde_json::to_value(v)
                        .map_err(|e| format!("Failed to serialize Vector: {e}"))?
                    })
                }
            }
            KhronosValue::Map(m) => {
                let mut map = serde_json::Map::new();
                for (k, v) in m.into_iter() {
                    if k == KHRONOS_VALUE_TYPE_KEY {
                        return Err(format!(
                            "Cannot use reserved key `{KHRONOS_VALUE_TYPE_KEY}` in map",
                        )
                        .into());
                    }
                    map.insert(k, v.into_serde_json_value(depth + 1, preserve_types)?);
                }
                serde_json::Value::Object(map)
            }
            KhronosValue::List(l) => {
                let mut list = Vec::with_capacity(l.len());
                for v in l.into_iter() {
                    list.push(v.into_serde_json_value(depth + 1, preserve_types)?);
                }
                serde_json::Value::Array(list)
            }
            // Special types have a __khronos_value_type field to identify them
            KhronosValue::Timestamptz(dt) => {
                if !preserve_types {
                    serde_json::to_value(dt)
                        .map_err(|e| format!("Failed to serialize DateTime: {e}"))?
                } else {
                    serde_json::json!({
                        KHRONOS_VALUE_TYPE_KEY: "timestamptz",
                        "value": serde_json::to_value(dt)
                        .map_err(|e| format!("Failed to serialize DateTime: {e}"))?
                    })
                }
            }
            KhronosValue::Interval(i) => {
                if !preserve_types {
                    serde_json::to_value(i)
                        .map_err(|e| format!("Failed to serialize Interval: {e}"))?
                } else {
                    serde_json::json!({
                        KHRONOS_VALUE_TYPE_KEY: "interval",
                        "value": serde_json::to_value(i)
                        .map_err(|e| format!("Failed to serialize Interval: {e}"))?
                    })
                }
            }
            KhronosValue::TimeZone(tz) => {
                if !preserve_types {
                    serde_json::to_value(tz)
                        .map_err(|e| format!("Failed to serialize TimeZone: {e}"))?
                } else {
                    serde_json::json!({
                        KHRONOS_VALUE_TYPE_KEY: "timezone",
                        "value": serde_json::to_value(tz)
                        .map_err(|e| format!("Failed to serialize TimeZone: {e}"))?
                    })
                }
            }
            KhronosValue::Null => serde_json::Value::Null,
        })
    }

    pub fn from_serde_json_value(
        value: serde_json::Value,
        depth: usize,
        preserve_types: bool,
    ) -> Result<Self, crate::Error> {
        if depth > 10 {
            return Err("Recursion limit exceeded".into());
        }

        Ok(match value {
            serde_json::Value::String(s) => KhronosValue::Text(s),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    KhronosValue::Integer(i)
                } else if let Some(u) = n.as_u64() {
                    KhronosValue::UnsignedInteger(u)
                } else if let Some(f) = n.as_f64() {
                    KhronosValue::Float(f)
                } else {
                    return Err("Invalid number type".into());
                }
            }
            serde_json::Value::Bool(b) => KhronosValue::Boolean(b),
            serde_json::Value::Object(mut m) => {
                if preserve_types {
                    if let Some(khronos_value_type) = m.get(KHRONOS_VALUE_TYPE_KEY) {
                        if let Some(khronos_value_type) = khronos_value_type.as_str() {
                            match khronos_value_type {
                                "buffer" => {
                                    let value = m.remove("value").ok_or("Missing value field")?;
                                    return Ok(KhronosValue::Buffer(
                                        serde_json::from_value(value).map_err(|e| {
                                            format!("Failed to deserialize Buffer: {e}")
                                        })?,
                                    ));
                                }
                                "vector" => {
                                    let value = m.remove("value").ok_or("Missing value field")?;
                                    return Ok(KhronosValue::Vector(
                                        serde_json::from_value(value).map_err(|e| {
                                            format!("Failed to deserialize Vector: {e}")
                                        })?,
                                    ));
                                }
                                "timestamptz" => {
                                    let value = m.remove("value").ok_or("Missing value field")?;
                                    return Ok(KhronosValue::Timestamptz(
                                        serde_json::from_value(value).map_err(|e| {
                                            format!("Failed to deserialize DateTime: {e}")
                                        })?,
                                    ));
                                }
                                "interval" => {
                                    let value = m.remove("value").ok_or("Missing value field")?;
                                    return Ok(KhronosValue::Interval(
                                        serde_json::from_value(value).map_err(|e| {
                                            format!("Failed to deserialize Interval: {e}")
                                        })?,
                                    ));
                                }
                                "timezone" => {
                                    let value = m.remove("value").ok_or("Missing value field")?;
                                    return Ok(KhronosValue::TimeZone(
                                        serde_json::from_value(value).map_err(|e| {
                                            format!("Failed to deserialize TimeZone: {e}")
                                        })?,
                                    ));
                                }
                                _ => {}
                            }
                        }
                    }
                }

                let mut map = indexmap::IndexMap::new();
                for (k, v) in m.into_iter() {
                    map.insert(
                        k,
                        Self::from_serde_json_value(v, depth + 1, preserve_types)?,
                    );
                }
                KhronosValue::Map(map)
            }
            serde_json::Value::Array(l) => {
                let mut list = Vec::with_capacity(l.len());
                for v in l.into_iter() {
                    list.push(Self::from_serde_json_value(v, depth + 1, preserve_types)?);
                }
                KhronosValue::List(list)
            }
            serde_json::Value::Null => KhronosValue::Null,
        })
    }

    /// Note: this is not the best performance-wise. In general, consider using `to_struct` to parse a KhronosValue to a struct etc.
    pub fn into_value<T: serde::de::DeserializeOwned>(self) -> Result<T, crate::Error> {
        let value = self.into_serde_json_value(0, true)?;
        T::deserialize(&value)
            .map_err(|e| crate::Error::from(format!("Failed to deserialize KhronosValue: {e}")))
    }

    /// Note: this is not the best performance-wise. In general, consider using `to_struct` to parse a KhronosValue to a struct etc.
    pub fn into_value_untyped<T: serde::de::DeserializeOwned>(self) -> Result<T, crate::Error> {
        let value = self.into_serde_json_value(0, false)?;
        T::deserialize(&value)
            .map_err(|e| crate::Error::from(format!("Failed to deserialize KhronosValue: {e}")))
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

impl Serialize for KhronosValue {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            KhronosValue::Null => serializer.serialize_unit(),
            KhronosValue::Boolean(b) => serializer.serialize_bool(*b),
            KhronosValue::Buffer(buf) => {
                // We need to preserve the KHRONOS_VALUE_TYPE_KEY field for deserialization
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry(KHRONOS_VALUE_TYPE_KEY, "buffer")?;
                map.serialize_entry("value", buf)?;
                map.end()
            }
            KhronosValue::Vector(v) => {
                // We need to preserve the KHRONOS_VALUE_TYPE_KEY field for deserialization
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry(KHRONOS_VALUE_TYPE_KEY, "vector")?;
                map.serialize_entry("value", v)?;
                map.end()
            }
            KhronosValue::Integer(i) => serializer.serialize_i64(*i),
            KhronosValue::UnsignedInteger(i) => serializer.serialize_u64(*i),
            KhronosValue::Float(f) => serializer.serialize_f64(*f),
            KhronosValue::Text(s) => serializer.serialize_str(s),
            KhronosValue::Timestamptz(dt) => {
                // We need to preserve the KHRONOS_VALUE_TYPE_KEY field for deserialization
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry(KHRONOS_VALUE_TYPE_KEY, "timestamptz")?;
                map.serialize_entry("value", dt)?;
                map.end()
            }
            KhronosValue::Interval(i) => {
                // We need to preserve the KHRONOS_VALUE_TYPE_KEY field for deserialization
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry(KHRONOS_VALUE_TYPE_KEY, "interval")?;
                map.serialize_entry("value", i)?;
                map.end()
            }
            KhronosValue::TimeZone(tz) => {
                // We need to preserve the KHRONOS_VALUE_TYPE_KEY field for deserialization
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry(KHRONOS_VALUE_TYPE_KEY, "timezone")?;
                map.serialize_entry("value", tz)?;
                map.end()
            }
            KhronosValue::Map(m) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            KhronosValue::List(v) => {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for value in v {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for KhronosValue {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        // First deserialize to a serde_json::Value
        let value = serde_json::Value::deserialize(deserializer)?;
        // Then convert to KhronosValue
        KhronosValue::from_serde_json_value(value, 0, true).map_err(serde::de::Error::custom)
    }
}