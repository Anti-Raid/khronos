use std::{borrow::Cow, collections::HashMap, fmt};

use mluau::prelude::*;
use serde::{Deserialize, Serialize, ser::{SerializeMap, SerializeSeq}};

use crate::core::typesext::MemoryVfs;

mod string_i64 {
    use serde::{de, Deserializer, Serializer};

    pub fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert i64 to string and serialize as a string
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct I64Visitor;

        impl<'de> de::Visitor<'de> for I64Visitor {
            type Value = i64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing an i64")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                value.parse().map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(I64Visitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KhronosValue {
    Text(Cow<'static, str>),
    Integer(i64),
    Int64(#[serde(with = "string_i64")] i64),
    Float(f64),
    Boolean(bool),
    Vector((f32, f32, f32)), // Luau vector
    Map(Vec<(KhronosValue, KhronosValue)>),
    StrMap(Box<HashMap<Cow<'static, str>, KhronosValue>>), // optimization on Map
    List(Vec<KhronosValue>),
    Timestamptz(chrono::DateTime<chrono::Utc>),
    Interval(chrono::Duration),
    TimeZone(chrono_tz::Tz),
    MemoryVfs(Box<HashMap<String, String>>),
    Nil(()),
    Null(()),
}

impl Default for KhronosValue {
    fn default() -> Self {
        KhronosValue::Nil(())
    }
}

impl KhronosValue {
    const ALLOWED_TYPES: &'static str = "DateTime | TimeDelta | TimeZone | Integer | UnsignedInteger | MemoryVfs";
    fn from_lua_impl(value: LuaValue, lua: &Lua, depth: usize) -> LuaResult<Self> {
        if depth > 20 {
            return Err(LuaError::FromLuaConversionError {
                from: "any",
                to: "KhronosValue".to_string(),
                message: Some("Recursion limit exceeded".to_string()),
            });
        }

        if value.is_null() {
            return Ok(KhronosValue::Null(()))
        }

        match value {
            LuaValue::String(s) => Ok(KhronosValue::Text(s.to_string_lossy().into())),
            LuaValue::Integer(i) => Ok(KhronosValue::Integer(i)),
            LuaValue::Int64(i) => Ok(KhronosValue::Int64(i)),
            LuaValue::Number(f) => Ok(KhronosValue::Float(f)),
            LuaValue::Boolean(b) => Ok(KhronosValue::Boolean(b)),
            LuaValue::Buffer(_) => {
                // We do not support storing values etc. yet
                Err(LuaError::FromLuaConversionError { from: "buffer", to: "KhronosValue".to_string(), message: Some("Cannot send/store buffers.".to_string()) })
            }
            LuaValue::Vector(v) => Ok(KhronosValue::Vector((v.x(), v.y(), v.z()))),
            LuaValue::Nil => Ok(KhronosValue::Nil(())),
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
                    let mut str_map: HashMap<Cow<'static, str>, KhronosValue> = HashMap::new();
                    let mut generic_map = Vec::new();
                    let mut is_pure_str_map = true;

                    for pair in table.pairs::<LuaValue, LuaValue>() {
                        let (k, v) = pair?;
                        let parsed_v = KhronosValue::from_lua_impl(v, lua, depth + 1)?;

                        if is_pure_str_map {
                            if let LuaValue::String(s) = &k {
                                str_map.insert(s.to_string_lossy().into(), parsed_v);
                                continue;
                            } else {
                                // We hit a non-string key! 
                                // Convert everything we gathered so far into the generic map.
                                is_pure_str_map = false;
                                for (str_k, val) in str_map.drain() {
                                    generic_map.push((KhronosValue::Text(str_k.into()), val));
                                }
                            }
                        }

                        // If we are no longer a pure string map, push to the generic map
                        let parsed_k = KhronosValue::from_lua_impl(k, lua, depth + 1)?;
                        generic_map.push((parsed_k, parsed_v));
                    }

                    if is_pure_str_map {
                        return Ok(KhronosValue::StrMap(Box::new(str_map)))
                    } else {
                        return Ok(KhronosValue::Map(generic_map))
                    }
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
                if let Ok(mut s_map) = ud.borrow_mut::<MemoryVfs>() {
                    // Take out the contents of the lazy string map 
                    let data = std::mem::take(&mut s_map.data);
                    return Ok(KhronosValue::MemoryVfs(data.into()));
                }

                Err(LuaError::FromLuaConversionError { from: "userdata", to: Self::ALLOWED_TYPES.to_string(), message: Some("Invalid UserData type.".to_string()) })
            }
            _ => Err(LuaError::FromLuaConversionError { from: value.type_name(), to: "KhronosValue".to_string(), message: Some("Unsupported Lua type for KhronosValue".to_string()) }),
        }
    }

    fn into_lua_impl(self, lua: &Lua, depth: usize) -> LuaResult<LuaValue> {
        if depth > 20 {
            return Err(LuaError::FromLuaConversionError {
                from: "any",
                to: "KhronosValue".to_string(),
                message: Some("Recursion limit exceeded".to_string()),
            });
        }

        match self {
            KhronosValue::Text(s) => Ok(LuaValue::String(lua.create_string(s.as_ref())?)),
            KhronosValue::Integer(i) => Ok(LuaValue::Integer(i)),
            KhronosValue::Int64(i) => Ok(LuaValue::Int64(i)),
            KhronosValue::Float(f) => Ok(LuaValue::Number(f)),
            KhronosValue::Boolean(b) => Ok(LuaValue::Boolean(b)),
            KhronosValue::Vector(v) => LuaVector::new(v.0, v.1, v.2).into_lua(lua),
            KhronosValue::Map(j) => {
                let table = lua.create_table_with_capacity(0, j.len())?;
                for (k, v) in j.into_iter() {
                    let k = k.into_lua_impl(lua, depth + 1)?;
                    let v = v.into_lua_impl(lua, depth + 1)?;
                    table.set(k, v)?;
                }
                Ok(LuaValue::Table(table))
            }
            KhronosValue::StrMap(j) => {
                let table = lua.create_table_with_capacity(0, j.len())?;
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
            KhronosValue::MemoryVfs(m) => MemoryVfs::new(*m).into_lua(lua),
            KhronosValue::Nil(_) => Ok(LuaValue::Nil),
            KhronosValue::Null(_) => Ok(LuaValue::NULL),
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

/// A small 'compressed' representation of a KhronosValue (at the cost of non-self-describability)
/// 
/// Only supports JSON/MessagePack
#[derive(Debug, Clone)]
pub struct CKhronosValue(pub KhronosValue);

/// Similar to CKhronosValue but takes a ref (internally, CKhronosValue just uses this for serializing)
pub struct CKhronosValueRef<'a>(&'a KhronosValue);
impl<'a> Serialize for CKhronosValueRef<'a> {    
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
    S: serde::Serializer {
        struct CompressedStrMap<'a>(&'a std::collections::HashMap<Cow<'static, str>, KhronosValue>);

        impl<'a> Serialize for CompressedStrMap<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(Some(self.0.len()))?;
                for (k, v) in self.0.iter() {
                    map.serialize_entry(k, &CKhronosValueRef(v))?;
                }
                map.end()
            }
        }

        struct CompressedMap<'a>(&'a [(KhronosValue, KhronosValue)]);

        impl<'a> Serialize for CompressedMap<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_seq(Some(self.0.len()))?;
                for (k, v) in self.0.iter() {
                    map.serialize_element(&(CKhronosValueRef(k), CKhronosValueRef(v)))?;
                }
                map.end()
            }
        }

        pub struct IntStr(i64);
        impl Serialize for IntStr {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                string_i64::serialize(&self.0, serializer)
            }
        }

        match self.0 {
            KhronosValue::Text(st) => serializer.serialize_str(st),
            KhronosValue::Nil(_) => serializer.serialize_unit(),
            KhronosValue::Boolean(b) => serializer.serialize_bool(*b),
            KhronosValue::List(l) => {
                let mut seq = serializer.serialize_seq(Some(l.len()))?;
                for v in l {
                    seq.serialize_element(&CKhronosValueRef(v))?; 
                }
                seq.end()
            }
            KhronosValue::Integer(i) => serializer.serialize_i64(*i),
            KhronosValue::Float(i) => serializer.serialize_f64(*i), // yes, this is *technically* lossy but CKhronosValue is allowed to be lossy anyways :)
            KhronosValue::Int64(i) => serializer.serialize_newtype_variant("Compressed", 0, "I64", &IntStr(*i)),
            KhronosValue::Map(m) => serializer.serialize_newtype_variant("Compressed", 1, "M", &CompressedMap(m.as_ref())),
            KhronosValue::StrMap(m) => serializer.serialize_newtype_variant("Compressed", 2, "#SM", &CompressedStrMap(m.as_ref())),
            KhronosValue::Vector(m) => serializer.serialize_newtype_variant("Compressed", 3, "Vec", m),
            KhronosValue::Timestamptz(m) => serializer.serialize_newtype_variant("Compressed", 4, "TS", m),
            KhronosValue::Interval(m) => serializer.serialize_newtype_variant("Compressed", 5, "IV", m),
            KhronosValue::TimeZone(m) => serializer.serialize_newtype_variant("Compressed", 6, "TZ", m),
            KhronosValue::MemoryVfs(m) => serializer.serialize_newtype_variant("Compressed", 7, "MVfs", m),
            KhronosValue::Null(m) => serializer.serialize_newtype_variant("Compressed", 8, "N", m),
        }    
    }
}

struct CKhronosVisitor;

impl<'de> serde::de::Visitor<'de> for CKhronosVisitor {
    type Value = CKhronosValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CKhronosValue payload")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CKhronosValue(KhronosValue::Boolean(v)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CKhronosValue(KhronosValue::Text(v.to_string().into())))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CKhronosValue(KhronosValue::Text(v.into())))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error, 
    {
        Ok(CKhronosValue(KhronosValue::Integer(v)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error, 
    {
        Ok(CKhronosValue(KhronosValue::Integer(v as i64)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error, 
    {
        Ok(CKhronosValue(KhronosValue::Float(v)))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CKhronosValue(KhronosValue::Nil(())))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CKhronosValue(KhronosValue::Null(())))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut list = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        
        while let Some(val) = seq.next_element::<CKhronosValue>()? {
            list.push(val.0);
        }
        
        Ok(CKhronosValue(KhronosValue::List(list)))
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let key: String = map.next_key()?
            .ok_or_else(|| serde::de::Error::custom("expected a tag key for non-primitive CKhronosValue but found an empty object"))?;

        let value = match key.as_str() {
            "I64" => {
                let s: String = map.next_value()?;
                KhronosValue::Int64(s.parse::<i64>().map_err(serde::de::Error::custom)?)
            }
            "M" => {
                let pairs: Vec<(CKhronosValue, CKhronosValue)> = map.next_value()?;
                // Strip the wrappers off the children
                let uncompressed = pairs.into_iter().map(|(k, v)| (k.0, v.0)).collect();
                KhronosValue::Map(uncompressed)
            }
            "#SM" => {
                let strmap: HashMap<String, CKhronosValue> = map.next_value()?;
                // Strip the wrappers off the children
                let uncompressed = strmap.into_iter().map(|(k, v)| (k.into(), v.0)).collect();
                KhronosValue::StrMap(Box::new(uncompressed))
            }
            "Vec" => KhronosValue::Vector(map.next_value()?),
            "TS" => KhronosValue::Timestamptz(map.next_value()?),
            "IV" => KhronosValue::Interval(map.next_value()?),
            "TZ" => KhronosValue::TimeZone(map.next_value()?),
            "MVfs" => KhronosValue::MemoryVfs(map.next_value()?),
            "N" => KhronosValue::Null(map.next_value()?),
            _ => return Err(serde::de::Error::custom(format!("Unknown tag: {}", key))),
        };

        Ok(CKhronosValue(value))
    }
}

impl Serialize for CKhronosValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
    S: serde::Serializer {
        CKhronosValueRef(&self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CKhronosValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(CKhronosVisitor)
    }
}

#[cfg(test)]
mod test_compressed {
    use std::collections::HashMap;

    use crate::utils::khronos_value::{CKhronosValue, CKhronosValueRef, KhronosValue};

    #[test]
    fn test_base() {
        let mut my_map = HashMap::new();
        my_map.insert("foo".into(), KhronosValue::Boolean(true));
        my_map.insert("carrot".into(), KhronosValue::Integer(23));
        my_map.insert("carrots".into(), KhronosValue::Float(23.45));
        my_map.insert("bar".into(), KhronosValue::Int64(284));
        my_map.insert("baz".into(), KhronosValue::List(vec![KhronosValue::Int64(333), KhronosValue::Nil(()), KhronosValue::Nil(()), KhronosValue::Null(()), KhronosValue::Text("Hello?".into())]));
        let my_kv = KhronosValue::StrMap(Box::new(my_map));
        let s = serde_json::to_string_pretty(&CKhronosValueRef(&my_kv)).expect("failed to serde");
        println!("{}", s);
        let deser = serde_json::from_str::<CKhronosValue>(&s).expect("failed to deser");
        println!("{:?}", deser)
    }
}