use mlua::prelude::*;
use serde::{Serialize, Deserialize};

const KHRONOS_VALUE_TYPE_KEY: &str = "___khronosValType___";

#[derive(Debug, Clone)]
pub enum KhronosValue {
    Text(String),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    Boolean(bool),
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

impl From<&str> for KhronosValue {
    fn from(value: &str) -> Self {
        KhronosValue::Text(value.to_string())
    }
}

impl From<String> for KhronosValue {
    fn from(value: String) -> Self {
        KhronosValue::Text(value)
    }
}
impl TryFrom<KhronosValue> for String {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Text(s) => Ok(s),
            _ => Err("KhronosValue is not a string".into()),
        }
    }
}
impl From<i8> for KhronosValue {
    fn from(value: i8) -> Self {
        KhronosValue::Integer(value.into())
    }
}
impl TryFrom<KhronosValue> for i8 {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Integer(i) => i.try_into().map_err(|_| "KhronosValue is not an i8".into()),
            _ => Err("KhronosValue is not an i8".into()),
        }
    }
}
impl From<i16> for KhronosValue {
    fn from(value: i16) -> Self {
        KhronosValue::Integer(value.into())
    }
}
impl TryFrom<KhronosValue> for i16 {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Integer(i) => i.try_into().map_err(|_| "KhronosValue is not an i16".into()),
            _ => Err("KhronosValue is not an i16".into()),
        }
    }
}
impl From<i32> for KhronosValue {
    fn from(value: i32) -> Self {
        KhronosValue::Integer(value.into())
    }
}
impl TryFrom<KhronosValue> for i32 {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Integer(i) => i.try_into().map_err(|_| "KhronosValue is not an i32".into()),
            _ => Err("KhronosValue is not an i32".into()),
        }
    }
}
impl From<i64> for KhronosValue {
    fn from(value: i64) -> Self {
        KhronosValue::Integer(value)
    }
}
impl TryFrom<KhronosValue> for i64 {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Integer(i) => Ok(i),
            _ => Err("KhronosValue is not an i64".into()),
        }
    }
}
impl From<u8> for KhronosValue {
    fn from(value: u8) -> Self {
        KhronosValue::UnsignedInteger(value.into())
    }
}
impl TryFrom<KhronosValue> for u8 {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::UnsignedInteger(i) => i.try_into().map_err(|_| "KhronosValue is not a u8".into()),
            _ => Err("KhronosValue is not a u8".into()),
        }
    }
}
impl From<u16> for KhronosValue {
    fn from(value: u16) -> Self {
        KhronosValue::UnsignedInteger(value.into())
    }
}
impl TryFrom<KhronosValue> for u16 {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::UnsignedInteger(i) => i.try_into().map_err(|_| "KhronosValue is not a u16".into()),
            _ => Err("KhronosValue is not a u16".into()),
        }
    }
}
impl From<u32> for KhronosValue {
    fn from(value: u32) -> Self {
        KhronosValue::UnsignedInteger(value.into())
    }
}
impl TryFrom<KhronosValue> for u32 {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::UnsignedInteger(i) => i.try_into().map_err(|_| "KhronosValue is not a u32".into()),
            _ => Err("KhronosValue is not a u32".into()),
        }
    }
}
impl From<u64> for KhronosValue {
    fn from(value: u64) -> Self {
        KhronosValue::UnsignedInteger(value)
    }
}
impl TryFrom<KhronosValue> for u64 {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::UnsignedInteger(i) => Ok(i),
            _ => Err("KhronosValue is not a u64".into()),
        }
    }
}

impl From<usize> for KhronosValue {
    fn from(value: usize) -> Self {
        KhronosValue::UnsignedInteger(value as u64)
    }
}

impl TryFrom<KhronosValue> for usize {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::UnsignedInteger(i) => i.try_into().map_err(|_| "KhronosValue is not a usize".into()),
            _ => Err("KhronosValue is not a usize".into()),
        }
    }
}

impl From<f32> for KhronosValue {
    fn from(value: f32) -> Self {
        KhronosValue::Float(value.into())
    }
}
impl TryFrom<KhronosValue> for f32 {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Float(f) => {
                if f > f32::MAX as f64 || f < f32::MIN as f64 {
                    return Err("KhronosValue is not a f32".into());
                }

                return Ok(f as f32);
            },
            _ => Err("KhronosValue is not a f32".into()),
        }
    }
}
impl From<f64> for KhronosValue {
    fn from(value: f64) -> Self {
        KhronosValue::Float(value)
    }
}
impl TryFrom<KhronosValue> for f64 {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Float(f) => Ok(f),
            _ => Err("KhronosValue is not a f64".into()),
        }
    }
}
impl From<bool> for KhronosValue {
    fn from(value: bool) -> Self {
        KhronosValue::Boolean(value)
    }
}
impl TryFrom<KhronosValue> for bool {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Boolean(b) => Ok(b),
            _ => Err("KhronosValue is not a bool".into()),
        }
    }
}

impl From<(f32, f32)> for KhronosValue {
    fn from(value: (f32, f32)) -> Self {
        KhronosValue::Vector((value.0, value.1, 0.0))
    }
}

impl TryFrom<KhronosValue> for (f32, f32) {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Vector(v) => Ok((v.0, v.1)),
            _ => Err("KhronosValue is not a vector".into()),
        }
    }
}

impl From<(f32, f32, f32)> for KhronosValue {
    fn from(value: (f32, f32, f32)) -> Self {
        KhronosValue::Vector(value)
    }
}

impl TryFrom<KhronosValue> for (f32, f32, f32) {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Vector(v) => Ok(v),
            _ => Err("KhronosValue is not a vector".into()),
        }
    }
}

impl From<chrono::DateTime<chrono::Utc>> for KhronosValue {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        KhronosValue::Timestamptz(value)
    }
}
impl TryFrom<KhronosValue> for chrono::DateTime<chrono::Utc> {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Timestamptz(dt) => Ok(dt),
            _ => Err("KhronosValue is not a DateTime".into()),
        }
    }
}
impl From<chrono::Duration> for KhronosValue {
    fn from(value: chrono::Duration) -> Self {
        KhronosValue::Interval(value)
    }
}
impl TryFrom<KhronosValue> for chrono::Duration {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Interval(dt) => Ok(dt),
            _ => Err("KhronosValue is not a Duration".into()),
        }
    }
}
impl From<chrono_tz::Tz> for KhronosValue {
    fn from(value: chrono_tz::Tz) -> Self {
        KhronosValue::TimeZone(value)
    }
}
impl TryFrom<KhronosValue> for chrono_tz::Tz {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::TimeZone(tz) => Ok(tz),
            _ => Err("KhronosValue is not a TimeZone".into()),
        }
    }
}

impl From<()> for KhronosValue {
    fn from(_: ()) -> Self {
        KhronosValue::Null
    }
}
impl TryFrom<KhronosValue> for () {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Null => Ok(()),
            _ => Err("KhronosValue is not a unit".into()),
        }
    }
}

impl<T> From<Option<T>> for KhronosValue
where
    T: Into<KhronosValue>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => KhronosValue::Null,
        }
    }
}

impl<T> TryFrom<KhronosValue> for Option<T>
where
    T: TryFrom<KhronosValue, Error = crate::Error>,
{
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Null => Ok(None),
            _ => Ok(Some(T::try_from(value)?)),
        }
    }
}

impl<T> From<Vec<T>> for KhronosValue
where
    T: Into<KhronosValue>,
{
    fn from(value: Vec<T>) -> Self {
        KhronosValue::List(value.into_iter().map(|v| v.into()).collect())
    }
}
impl<T> TryFrom<KhronosValue> for Vec<T>
where
    T: TryFrom<KhronosValue, Error = crate::Error>,
{
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::List(l) => l.into_iter().map(T::try_from).collect(),
            _ => Err("KhronosValue is not a list".into()),
        }
    }
}

impl<T> From<indexmap::IndexMap<String, T>> for KhronosValue
where
    T: Into<KhronosValue>,
{
    fn from(value: indexmap::IndexMap<String, T>) -> Self {
        KhronosValue::Map(value.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}
impl<T> TryFrom<KhronosValue> for indexmap::IndexMap<String, T>
where
    T: TryFrom<KhronosValue, Error = crate::Error>,
{
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Map(m) => m.into_iter().map(|(k, v)| Ok((k, T::try_from(v)?))).collect(),
            _ => Err("KhronosValue is not a map".into()),
        }
    }
}
impl From<std::collections::HashMap<String, KhronosValue>> for KhronosValue {
    fn from(value: std::collections::HashMap<String, KhronosValue>) -> Self {
        KhronosValue::Map(value.into_iter().collect())
    }
}
impl TryFrom<KhronosValue> for std::collections::HashMap<String, KhronosValue> {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        match value {
            KhronosValue::Map(m) => Ok(m.into_iter().collect()),
            _ => Err("KhronosValue is not a map".into()),
        }
    }
}

/// Simple struct to allow for embedding a serde-able type that can be converted to/from a KhronosValue using TryFrom
pub struct SerdeBlob<T: Serialize + for<'de> Deserialize<'de>>(T);

impl<T: Serialize + for<'de> Deserialize<'de>> std::ops::Deref for SerdeBlob<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Serialize + for<'de> Deserialize<'de>> std::ops::DerefMut for SerdeBlob<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Serialize + for<'de> Deserialize<'de>> SerdeBlob<T> {
    pub fn new(value: T) -> Self {
        SerdeBlob(value)
    }

    pub fn take(self) -> T {
        self.0
    }
}

impl<T: Serialize + for<'de> Deserialize<'de>> TryFrom<SerdeBlob<T>> for KhronosValue {
    type Error = crate::Error;
    fn try_from(value: SerdeBlob<T>) -> Result<Self, Self::Error> {
        let value = serde_json::to_value(value.0)?;
        let value = KhronosValue::from_serde_json_value(value, 0)?;
        Ok(value)
    }
}

impl<T: Serialize + for<'de> Deserialize<'de>> TryFrom<KhronosValue> for SerdeBlob<T> {
    type Error = crate::Error;
    fn try_from(value: KhronosValue) -> Result<Self, Self::Error> {
        let serde_json_value = value.into_serde_json_value(0, false)?;
        let value = T::deserialize(serde_json_value).map_err(|e| {
            crate::Error::from(format!("Failed to deserialize SerdeBlob: {}", e))
        })?;
        Ok(SerdeBlob(value))
    }
}

/// Macro to cheaply create a KhronosValue
///
/// value!(1, 2, 3) will create a KhronosValue::List(vec![
///     KhronosValue::Integer(1),
///     KhronosValue::Integer(2),
///     KhronosValue::Integer(3),
/// ]);
///
/// and value!(1) will create a KhronosValue::Integer(1)
/// and value!("hello" => "world") will create a KhronosValue::Map(indexmap!{"hello".to_string() => KhronosValue::Text("world".to_string())})
#[macro_export]
macro_rules! value {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = indexmap::IndexMap::new();
            $(
                map.insert($key.to_string(), ($value).into());
            )*
            $crate::utils::khronos_value::KhronosValue::Map(map)
        }
    };
    ($value:expr) => {
        Into::<$crate::utils::khronos_value::KhronosValue>::into($value)
    };
    ($valuea:expr, $($value:expr),+) => {
        {
            let mut list = Vec::new();
            list.push(($valuea).into());
            $(
                list.push(($value).into());
            )*
            $crate::utils::khronos_value::KhronosValue::List(list)
        }
    };
}

impl KhronosValue {
    pub fn kind(&self) -> &'static str {
        match self {
            KhronosValue::Text(_) => "text",
            KhronosValue::Integer(_) => "integer",
            KhronosValue::UnsignedInteger(_) => "unsigned_integer",
            KhronosValue::Float(_) => "float",
            KhronosValue::Boolean(_) => "boolean",
            KhronosValue::Vector(_) => "vector",
            KhronosValue::Map(_) => "map",
            KhronosValue::List(_) => "list",
            KhronosValue::Timestamptz(_) => "timestamptz",
            KhronosValue::Interval(_) => "interval",
            KhronosValue::TimeZone(_) => "timezone",
            KhronosValue::Null => "null",
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, KhronosValue::Null)
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            KhronosValue::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            KhronosValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_unsigned_integer(&self) -> Option<u64> {
        match self {
            KhronosValue::UnsignedInteger(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            KhronosValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            KhronosValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_vector(&self) -> Option<(f32, f32, f32)> {
        match self {
            KhronosValue::Vector(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&indexmap::IndexMap<String, KhronosValue>> {
        match self {
            KhronosValue::Map(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<KhronosValue>> {
        match self {
            KhronosValue::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_timestamptz(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        match self {
            KhronosValue::Timestamptz(dt) => Some(*dt),
            _ => None,
        }
    }

    pub fn as_interval(&self) -> Option<chrono::Duration> {
        match self {
            KhronosValue::Interval(dt) => Some(*dt),
            _ => None,
        }
    }

    pub fn as_timezone(&self) -> Option<chrono_tz::Tz> {
        match self {
            KhronosValue::TimeZone(tz) => Some(tz.clone()),
            _ => None,
        }
    }

    fn from_lua_impl(value: LuaValue, lua: &Lua, depth: usize) -> LuaResult<Self> {
        if depth > 10 {
            return Err(LuaError::FromLuaConversionError { from: "any", to: "KhronosValue".to_string(), message: Some("Recursion limit exceeded".to_string()) });
        }

        match value {
            LuaValue::String(s) => Ok(KhronosValue::Text(s.to_string_lossy().to_string())),
            LuaValue::Integer(i) => Ok(KhronosValue::Integer(i)),
            LuaValue::Number(f) => Ok(KhronosValue::Float(f)),
            LuaValue::Boolean(b) => Ok(KhronosValue::Boolean(b)),
            LuaValue::Vector(v) => Ok(KhronosValue::Vector((v.x(), v.y(), v.z()))),
            LuaValue::Nil => Ok(KhronosValue::Null),
            LuaValue::Table(table) => {
                if table.raw_len() == 0 {
                    // Map
                    let mut map = indexmap::IndexMap::new();
                    for pair in table.pairs::<String, LuaValue>() {
                        let (k, v) = pair?;
                        let v = KhronosValue::from_lua_impl(v, lua, depth+1)?;
                        map.insert(k, v);
                    }
                    return Ok(KhronosValue::Map(map));
                }
                // Check if the table is a list
                let mut list = Vec::new();
                for v in table.sequence_values::<LuaValue>() {
                    let v = v?;
                    let v = KhronosValue::from_lua_impl(v, lua, depth+1)?;
                    list.push(v);
                }

                Ok(KhronosValue::List(list))
            }
            LuaValue::UserData(ud) => {
                if let Ok(dt) = ud.borrow::<crate::plugins::antiraid::datetime::DateTime<chrono_tz::Tz>>() {
                    return Ok(KhronosValue::Timestamptz(dt.dt.with_timezone(&chrono::Utc)));
                }
                if let Ok(delta) = ud.borrow::<crate::plugins::antiraid::datetime::TimeDelta>() {
                    return Ok(KhronosValue::Interval(delta.timedelta));
                }
                if let Ok(tz) = ud.borrow::<crate::plugins::antiraid::datetime::Timezone>() {
                    return Ok(KhronosValue::TimeZone(tz.tz.clone()));
                }
                if let Ok(i_64) = ud.borrow::<crate::plugins::antiraid::typesext::I64>() {
                    return Ok(KhronosValue::Integer(i_64.0));
                }
                if let Ok(u_64) = ud.borrow::<crate::plugins::antiraid::typesext::U64>() {
                    return Ok(KhronosValue::UnsignedInteger(u_64.0));
                }

                return Err(LuaError::FromLuaConversionError { from: "userdata", to: "DateTime | TimeDelta | TimeZone".to_string(), message: Some("Invalid UserData type. Only DateTime, TimeDelta and TimeZone is supported at this time".to_string()) });
            }
            _ => Err(LuaError::FromLuaConversionError { from: "any", to: "KhronosValue".to_string(), message: Some("Invalid type".to_string()) }),
        }
    }

    fn into_lua_impl(self, lua: &Lua, depth: usize) -> LuaResult<LuaValue> {
        if depth > 10 {
            return Err(LuaError::FromLuaConversionError { from: "any", to: "KhronosValue".to_string(), message: Some("Recursion limit exceeded".to_string()) });
        }

        match self {
            KhronosValue::Text(s) => Ok(LuaValue::String(lua.create_string(&s)?)),
            KhronosValue::Integer(i) => {
                // If i is above/below the 52 bit precision limit, use a typesext.I64
                let min_luau_integer = -9007199254740991; // 2^53 - 1
                let max_luau_integer = 9007199254740991; // 2^53 - 1
                if i > max_luau_integer || i < min_luau_integer {
                    crate::plugins::antiraid::typesext::I64(i).into_lua(lua)
                } else {
                    Ok(LuaValue::Integer(i))
                }
            },
            KhronosValue::UnsignedInteger(i) => crate::plugins::antiraid::typesext::U64(i).into_lua(lua), // An UnsignedInteger can only be created through explicit U64 parse
            KhronosValue::Float(f) => Ok(LuaValue::Number(f)),
            KhronosValue::Boolean(b) => Ok(LuaValue::Boolean(b)),
            KhronosValue::Vector(v) => LuaVector::new(v.0, v.1, v.2).into_lua(lua),
            KhronosValue::Map(j) => {
                let table = lua.create_table()?;
                for (k, v) in j.into_iter() {
                    let v = v.into_lua_impl(lua, depth+1)?;
                    table.set(k, v)?;
                }
                Ok(LuaValue::Table(table))
            }
            KhronosValue::List(l) => {
                let table = lua.create_table()?;
                for v in l.into_iter() {
                    let v = v.into_lua_impl(lua, depth+1)?;
                    table.set(table.raw_len() + 1, v)?;
                }
                Ok(LuaValue::Table(table))
            }
            KhronosValue::Timestamptz(dt) => crate::plugins::antiraid::datetime::DateTime::<chrono_tz::Tz>::from_utc(dt).into_lua(lua),
            KhronosValue::Interval(i) => crate::plugins::antiraid::datetime::TimeDelta::new(i).into_lua(lua),
            KhronosValue::TimeZone(tz) => crate::plugins::antiraid::datetime::Timezone::new(tz).into_lua(lua),
            KhronosValue::Null => Ok(LuaValue::Nil),
        }
    }

    pub fn into_serde_json_value(self, depth: usize, preserve_types: bool) -> Result<serde_json::Value, crate::Error> {
        if depth > 10 {
            return Err("Recursion limit exceeded".into());
        }

        Ok(match self {
            KhronosValue::Text(s) => serde_json::Value::String(s),
            KhronosValue::Integer(i) => serde_json::Value::Number(serde_json::Number::from(i)),
            KhronosValue::UnsignedInteger(i) => serde_json::Value::Number(serde_json::Number::from(i)),
            KhronosValue::Float(f) => serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap()),
            KhronosValue::Boolean(b) => serde_json::Value::Bool(b),
            KhronosValue::Vector(v) => {
                if !preserve_types {
                    serde_json::to_value(v)
                        .map_err(|e| format!("Failed to serialize Vector: {}", e))?
                } else {
                    serde_json::json!({
                        KHRONOS_VALUE_TYPE_KEY: "vector",
                        "value": serde_json::to_value(v)
                        .map_err(|e| format!("Failed to serialize Vector: {}", e))?
                    })
                }
            }
            KhronosValue::Map(m) => {
                let mut map = serde_json::Map::new();
                for (k, v) in m.into_iter() {
                    if k == KHRONOS_VALUE_TYPE_KEY {
                        return Err(format!("Cannot use reserved key `{}` in map", KHRONOS_VALUE_TYPE_KEY).into());
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
                        .map_err(|e| format!("Failed to serialize DateTime: {}", e))?
                } else {
                    serde_json::json!({
                        KHRONOS_VALUE_TYPE_KEY: "timestamptz",
                        "value": serde_json::to_value(dt)
                        .map_err(|e| format!("Failed to serialize DateTime: {}", e))?
                    })
                }
            },
            KhronosValue::Interval(i) => {
                if !preserve_types {
                    serde_json::to_value(i)
                        .map_err(|e| format!("Failed to serialize Interval: {}", e))?
                } else {
                    serde_json::json!({
                        KHRONOS_VALUE_TYPE_KEY: "interval",
                        "value": serde_json::to_value(i)
                        .map_err(|e| format!("Failed to serialize Interval: {}", e))?
                    })
                }
            },
            KhronosValue::TimeZone(tz) => {
                if !preserve_types {
                    serde_json::to_value(tz)
                        .map_err(|e| format!("Failed to serialize TimeZone: {}", e))?
                } else {
                    serde_json::json!({
                        KHRONOS_VALUE_TYPE_KEY: "timezone",
                        "value": serde_json::to_value(tz)
                        .map_err(|e| format!("Failed to serialize TimeZone: {}", e))?
                    })
                }
            },
            KhronosValue::Null => serde_json::Value::Null,
        })
    }

    pub fn from_serde_json_value(value: serde_json::Value, depth: usize) -> Result<Self, crate::Error> {
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
            serde_json::Value::Object(m) => {
                if let Some(khronos_value_type) = m.get(KHRONOS_VALUE_TYPE_KEY) {
                    if let Some(khronos_value_type) = khronos_value_type.as_str() {
                        match khronos_value_type {
                            "vector" => {
                                let value = m.get("value").ok_or("Missing value field")?;
                                return Ok(KhronosValue::Vector(serde_json::from_value(value.clone()).map_err(|e| format!("Failed to deserialize Vector: {}", e))?));
                            }
                            "timestamptz" => {
                                let value = m.get("value").ok_or("Missing value field")?;
                                return Ok(KhronosValue::Timestamptz(serde_json::from_value(value.clone()).map_err(|e| format!("Failed to deserialize DateTime: {}", e))?));
                            }
                            "interval" => {
                                let value = m.get("value").ok_or("Missing value field")?;
                                return Ok(KhronosValue::Interval(serde_json::from_value(value.clone()).map_err(|e| format!("Failed to deserialize Interval: {}", e))?));
                            }
                            "timezone" => {
                                let value = m.get("value").ok_or("Missing value field")?;
                                return Ok(KhronosValue::TimeZone(serde_json::from_value(value.clone()).map_err(|e| format!("Failed to deserialize TimeZone: {}", e))?));
                            }
                            _ => {}
                        }
                    }
                }

                let mut map = indexmap::IndexMap::new();
                for (k, v) in m.into_iter() {
                    map.insert(k, Self::from_serde_json_value(v, depth + 1)?);
                }
                KhronosValue::Map(map)
            }
            serde_json::Value::Array(l) => {
                let mut list = Vec::with_capacity(l.len());
                for v in l.into_iter() {
                    list.push(Self::from_serde_json_value(v, depth + 1)?);
                }
                KhronosValue::List(list)
            }
            serde_json::Value::Null => KhronosValue::Null,
        })
    }

    /// Note: this is not the best performance-wise. In general, consider using `to_struct` to parse a KhronosValue to a struct etc.
    pub fn into_value<T: serde::de::DeserializeOwned>(self) -> Result<T, crate::Error> {
        let value = self.into_serde_json_value(0, true)?;
        Ok(T::deserialize(&value).map_err(|e| {
            crate::Error::from(format!("Failed to deserialize KhronosValue: {}", e))
        })?)
    }

    pub fn visit<T: KhronosValueVisitor>(&self, visitor: &mut T) -> Result<(), crate::Error> {
        match self {
            KhronosValue::Text(s) => visitor.visit_text(s),
            KhronosValue::Integer(i) => visitor.visit_integer(*i),
            KhronosValue::UnsignedInteger(i) => visitor.visit_unsigned_integer(*i),
            KhronosValue::Float(f) => visitor.visit_float(*f),
            KhronosValue::Boolean(b) => visitor.visit_boolean(*b),
            KhronosValue::Vector(v) => visitor.visit_vector(*v),
            KhronosValue::Map(m) => {
                visitor.visit_map(m)?;  
                for (k, v) in m {
                    visitor.visit_map_entry(k, v)?;
                    v.visit(visitor)?;
                    visitor.visit_map_entry_end(k, v)?;
                }
                visitor.visit_map_end(m)
            },
            KhronosValue::List(l) => {
                visitor.visit_list(l)?;
                for v in l {
                    visitor.visit_list_entry(v)?;
                    v.visit(visitor)?;
                    visitor.visit_list_entry_end(v)?;
                }
                visitor.visit_list_end(l)
            },
            KhronosValue::Timestamptz(dt) => visitor.visit_timestamptz(dt),
            KhronosValue::Interval(i) => visitor.visit_interval(i),
            KhronosValue::TimeZone(tz) => visitor.visit_timezone(tz),
            KhronosValue::Null => visitor.visit_null(),
        }
    }
}

/// A trait for visiting KhronosValue types
/// This is used to implement the visitor pattern for KhronosValue
pub trait KhronosValueVisitor {
    fn visit_text(&mut self, _value: &str) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_integer(&mut self, _value: i64) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_unsigned_integer(&mut self, _value: u64) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_float(&mut self, _value: f64) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_boolean(&mut self, _value: bool) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_vector(&mut self, _value: (f32, f32, f32)) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_map(&mut self, _value: &indexmap::IndexMap<String, KhronosValue>) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_map_entry(&mut self, _key: &str, _value: &KhronosValue) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_map_entry_end(&mut self, _key: &str, _value: &KhronosValue) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_map_end(&mut self, _value: &indexmap::IndexMap<String, KhronosValue>) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_list(&mut self, _value: &Vec<KhronosValue>) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_list_entry(&mut self, _value: &KhronosValue) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_list_entry_end(&mut self, _value: &KhronosValue) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_list_end(&mut self, _value: &Vec<KhronosValue>) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_timestamptz(&mut self, _value: &chrono::DateTime<chrono::Utc>) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_interval(&mut self, _value: &chrono::Duration) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_timezone(&mut self, _value: &chrono_tz::Tz) -> Result<(), crate::Error> {
        Ok(())
    }
    fn visit_null(&mut self) -> Result<(), crate::Error> {
        Ok(())
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
        KhronosValue::from_serde_json_value(value, 0).map_err(serde::de::Error::custom)
    }
}

/// Macro to convert a KhronosValue to a struct T where every field in T can be converted from a KhronosValue
/// This is a convenience macro to avoid having to write the conversion code manually
///
/// Example:
///
/// ```
/// use khronos_runtime::to_struct;
/// to_struct!(
///     #[derive(Debug, PartialEq, Clone)] // Add derives here
///     // Add other attributes like #[serde(...)] if needed
///     pub struct MyData { // Use standard struct definition syntax
///         pub name: String,
///         pub value: i64,
///         is_active: bool, // Fields can be private
///         maybe_float: Option<f64>,
///     }
/// );
/// ```
///
#[macro_export]
macro_rules! to_struct {
    (
        // Capture outer attributes (like #[derive(...)])
        $(#[$outer:meta])*
        // Capture visibility (pub, pub(crate), etc.), struct keyword, and struct name
        $vis:vis struct $struct_name:ident {
            // Capture each field: visibility, name, colon, type
            // $( ... ),* means repeat zero or more times, separated by commas
            $(
                $field_vis:vis $field_name:ident : $field_type:ty
            ),* // The fields themselves
            $(,)? // Optionally allow a trailing comma after the last field
        }
    ) => {
        // --- Generate the struct definition ---
        $(#[$outer])* // Apply the captured outer attributes
        $vis struct $struct_name {
            $(
                $field_vis $field_name: $field_type,
            )*
        }

        // --- Generate the From<KhronosValue> implementation ---
        impl std::convert::TryFrom<$crate::utils::khronos_value::KhronosValue> for $struct_name {
            type Error = Box<dyn std::error::Error + Send + Sync>;
            fn try_from(value: $crate::utils::khronos_value::KhronosValue) -> Result<Self, Self::Error> {
                match value {
                    $crate::utils::khronos_value::KhronosValue::Map(mut map) => { // Take ownership and make map mutable
                        Ok(Self {
                            // Iterate through the captured fields to generate initializers
                            $(
                                $field_name: {
                                    // Get the field name as a string literal
                                    let key = stringify!($field_name);
                                    // Attempt to remove the value from the map using the field name as the key
                                    let field_value = match map.swap_remove(key) {
                                        Some(v) => v,
                                        None => $crate::utils::khronos_value::KhronosValue::Null, // If the key is not found, use Null
                                    };
                                    let field_type_kind = field_value.kind();
                                    // Convert the retrieved KhronosValue into the expected field type
                                    // This relies on `$field_type` implementing `From<KhronosValue>`
                                    <$field_type>::try_from(field_value).map_err(|e| {
                                        // If conversion fails, panic with a message
                                        format!(
                                            "Failed to convert field '{}' in KhronosValue::Map for struct {}: {}, type: {}",
                                            key,
                                            stringify!($struct_name), // Name of the struct being built
                                            e, // The error from the conversion
                                            field_type_kind, // The kind of the KhronosValue
                                        )
                                    })?
                                },
                            )*
                        })
                    }
                    // If the input KhronosValue is not a Map, panic.
                    other => return Err(
                        format!(
                            "Expected KhronosValue::Map to convert to struct {}, found {:?}",
                            stringify!($struct_name), // Name of the target struct
                            other // The unexpected KhronosValue variant
                        ).into()
                    ),
                }
            }
        }

        // --- Generate From<Struct> for KhronosValue ---
        impl std::convert::From<$struct_name> for $crate::utils::khronos_value::KhronosValue { // Assuming KhronosValue is in the crate root
            fn from(value: $struct_name) -> Self {
                // Use indexmap::IndexMap directly or ensure it's in scope
                let mut map = indexmap::IndexMap::new();
                $(
                    // TODO/MAYBE: Ideally respect #[serde(rename = "...")] for the key name
                    // For now, it uses the Rust field name.
                    let key = stringify!($field_name).to_string();
                    // Use .into() which requires From<FieldType> for KhronosValue / Into trait bound
                    let khronos_val: $crate::utils::khronos_value::KhronosValue = value.$field_name.into(); // Assuming KhronosValue is in the crate root
                    map.insert(key, khronos_val);
                )*
                $crate::utils::khronos_value::KhronosValue::Map(map) // Assuming KhronosValue is in the crate root
            }
        }       
    };
}


#[cfg(test)]
mod test_value_macro {
    use super::*;
    #[test]
    fn test_value_macro() {
        let v = value!(1, 2, 3);
        assert_eq!(v.as_list().unwrap().len(), 3);
        assert_eq!(v.as_list().unwrap()[0].as_integer().unwrap(), 1);
        assert_eq!(v.as_list().unwrap()[1].as_integer().unwrap(), 2);
        assert_eq!(v.as_list().unwrap()[2].as_integer().unwrap(), 3);

        let v = value!("hello" => "world");
        assert_eq!(v.as_map().unwrap().len(), 1);
        assert_eq!(v.as_map().unwrap()["hello"].as_string().unwrap(), "world");

        let v = value!("world");
        assert_eq!(v.as_string().unwrap(), "world");

        let kv = KhronosValue::Text("hello".to_string());
        let v = value!(kv.clone());
        assert_eq!(v.as_string().unwrap(), "hello");
        let v2 = value!(1 => kv);
        assert_eq!(v2.as_map().unwrap().len(), 1);
        assert_eq!(v2.as_map().unwrap()[&"1".to_string()].as_string().unwrap(), "hello");
    }
}

#[cfg(test)]
mod test_to_struct {
    use serde::Serialize;
    use serde::Deserialize;

    to_struct!(
        #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct MyData {
            pub name: String,
            pub value: i64,
            is_active: bool,
            maybe_float: Option<f64>,
            a_list: Vec<i64>,
            meow: Option<String>,
        }
    );

    #[test]
    fn test_to_struct() {
        let kv = value!(
            "name".to_string() => "test".to_string(),
            "value".to_string() => 42,
            "is_active".to_string() => true,
            "maybe_float".to_string() => 3.14,
            "a_list".to_string() => value!(1, 2, 3)
        );

        let my_data: MyData = kv.try_into().unwrap();
        assert_eq!(my_data.name, "test");
        assert_eq!(my_data.value, 42);
        assert_eq!(my_data.is_active, true);
        assert_eq!(my_data.maybe_float, Some(3.14));
        assert_eq!(my_data.a_list, vec![1, 2, 3]);
        println!("{:?}", value!(my_data))
    }
}