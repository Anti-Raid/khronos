use mlua::prelude::*;
use serenity::async_trait;
use std::{future::Future, pin::Pin};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeString {
    inner_str: String
}

impl SafeString {
    pub fn is_safe(s: &str) -> bool {
        s.chars().all(|c| c.is_alphanumeric() || c == '_')
    }

    pub fn new(s: String) -> Option<Self> {
        if !Self::is_safe(&s) {
            return None;
        }

        Some(Self {
            inner_str: s
        })
    }
}

impl FromLua for SafeString {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let s = String::from_lua(value, lua)?;
        let Some(safe_string) = SafeString::new(s) else {
            return Err(LuaError::FromLuaConversionError { from: "any", to: "SafeString".to_string(), message: Some("SafeStrings can only contain alphanumeric characters or underscores".to_string()) });
        };
        Ok(safe_string)
    }
}

impl IntoLua for SafeString {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        String::into_lua(self.inner_str, lua)
    }
}

impl std::fmt::Display for SafeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner_str)
    }
}   

impl std::ops::Deref for SafeString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner_str
    }
}

impl serde::Serialize for SafeString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Ensure the string only contains either alphanumeric characters or underscores
        if !SafeString::is_safe(&self.inner_str) {
            return Err(serde::ser::Error::custom("SafeStrings can only contain alphanumeric characters or underscores"));
        }

        serializer.serialize_str(&self.inner_str)
    }
}

impl<'de> serde::Deserialize<'de> for SafeString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        SafeString::new(s).ok_or(serde::de::Error::custom("SafeStrings can only contain alphanumeric characters or underscores"))
    }
}

/// A standard type for filters
#[derive(Clone, Debug)]
pub enum Filter {
    EqualCond {
        field_name: SafeString,
        value: Box<DataStoreValue>,
    },
    NotEqualCond {
        field_name: SafeString,
        value: Box<DataStoreValue>,
    },
    GreaterCond {
        field_name: SafeString,
        value: Box<DataStoreValue>,
    },
    GreaterEqualCond {
        field_name: SafeString,
        value: Box<DataStoreValue>,
    },
    LessCond {
        field_name: SafeString,
        value: Box<DataStoreValue>,
    },
    LessEqualCond {
        field_name: SafeString,
        value: Box<DataStoreValue>,
    },
    InCond {
        field_name: SafeString,
        values: Vec<Box<DataStoreValue>>,
    },
    NotInCond {
        field_name: SafeString,
        values: Vec<Box<DataStoreValue>>,
    },
    LikeCond {
        field_name: SafeString,
        value: Box<DataStoreValue>,
    },
    NotLikeCond {
        field_name: SafeString,
        value: Box<DataStoreValue>,
    },
    ILikeCond {
        field_name: SafeString,
        value: Box<DataStoreValue>,
    },
    NotILikeCond {
        field_name: SafeString,
        value: Box<DataStoreValue>,
    },
    Group {
        filters: Vec<FilterWithContinuation>,
    }
}

impl IntoLua for Filter {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        match self {
            Filter::EqualCond { field_name, value } => {
                table.set("type", "Equal")?;
                table.set("field_name", field_name)?;
                table.set("value", *value)?;
            }
            Filter::NotEqualCond { field_name, value } => {
                table.set("type", "NotEqual")?;
                table.set("field_name", field_name)?;
                table.set("value", *value)?;
            }
            Filter::GreaterCond { field_name, value } => {
                table.set("type", "Greater")?;
                table.set("field_name", field_name)?;
                table.set("value", *value)?;
            }
            Filter::GreaterEqualCond { field_name, value } => {
                table.set("type", "GreaterEqual")?;
                table.set("field_name", field_name)?;
                table.set("value", *value)?;
            }
            Filter::LessCond { field_name, value } => {
                table.set("type", "Less")?;
                table.set("field_name", field_name)?;
                table.set("value", *value)?;
            }
            Filter::LessEqualCond { field_name, value } => {
                table.set("type", "LessEqual")?;
                table.set("field_name", field_name)?;
                table.set("value", *value)?;
            }
            Filter::InCond { field_name, values } => {
                table.set("type", "In")?;
                table.set("field_name", field_name)?;
                table.set("values", values)?;
            }
            Filter::NotInCond { field_name, values } => {
                table.set("type", "NotIn")?;
                table.set("field_name", field_name)?;
                table.set("values", values)?;
            }
            Filter::LikeCond { field_name, value } => {
                table.set("type", "Like")?;
                table.set("field_name", field_name)?;
                table.set("value", *value)?;
            }
            Filter::NotLikeCond { field_name, value } => {
                table.set("type", "NotLike")?;
                table.set("field_name", field_name)?;
                table.set("value", *value)?;
            }
            Filter::ILikeCond { field_name, value } => {
                table.set("type", "ILike")?;
                table.set("field_name", field_name)?;
                table.set("value", *value)?;
            }
            Filter::NotILikeCond { field_name, value } => {
                table.set("type", "NotILike")?;
                table.set("field_name", field_name)?;
                table.set("value", *value)?;
            }
            Filter::Group { filters } => {
                table.set("type", "Group")?;
                let filters_table = lua.create_table()?;
                for (i, filter) in filters.into_iter().enumerate() {
                    let filter_lua = filter.filter.into_lua(lua)?;
                    filters_table.set(i + 1, filter_lua)?;
                }
                table.set("filters", filters_table)?;
            }
        }
        Ok(LuaValue::Table(table))
    }
}

impl FromLua for Filter {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        let LuaValue::Table(table) = value else {
            return Err(LuaError::FromLuaConversionError { from: "any", to: "Filter".to_string(), message: Some("Expected a table".to_string()) });
        };

        let filter_type: String = table.get("type")?;
        let data = table.get("data")?;

        Self::from_lua_value(filter_type, data)
    }
}

impl Filter {
    pub fn from_lua_value(filter_type: String, data: LuaValue) -> LuaResult<Self> {
        let filter = match filter_type.as_ref() {
            "Equal" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let value = data.get::<DataStoreValue>("value")?;
                Filter::EqualCond {
                    field_name,
                    value: value.into(),
                }
            },
            "NotEqual" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let value = data.get::<DataStoreValue>("value")?;
                Filter::NotEqualCond {
                    field_name,
                    value: value.into(),
                }
            },
            "Greater" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let value = data.get::<DataStoreValue>("value")?;
                Filter::GreaterCond {
                    field_name,
                    value: value.into(),
                }
            },
            "GreaterEqual" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let value = data.get::<DataStoreValue>("value")?;
                Filter::GreaterEqualCond {
                    field_name,
                    value: value.into(),
                }
            },
            "Less" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let value = data.get::<DataStoreValue>("value")?;
                Filter::LessCond {
                    field_name,
                    value: value.into(),
                }
            },
            "LessEqual" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let value = data.get::<DataStoreValue>("value")?;
                Filter::LessEqualCond {
                    field_name,
                    value: value.into(),
                }
            },
            "In" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let values = data.get::<Vec<DataStoreValue>>("values")?;
                Filter::InCond {
                    field_name,
                    values: values.into_iter().map(|x| x.into()).collect(),
                }
            },
            "NotIn" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let values = data.get::<Vec<DataStoreValue>>("values")?;
                Filter::NotInCond {
                    field_name,
                    values: values.into_iter().map(|x| x.into()).collect(),
                }
            },
            "Like" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let value = data.get::<DataStoreValue>("value")?;
                Filter::LikeCond {
                    field_name,
                    value: value.into(),
                }
            },
            "NotLike" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let value = data.get::<DataStoreValue>("value")?;
                Filter::NotLikeCond {
                    field_name,
                    value: value.into(),
                }
            },
            "ILike" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let value = data.get::<DataStoreValue>("value")?;
                Filter::ILikeCond {
                    field_name,
                    value: value.into(),
                }
            },
            "NotILike" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let field_name = data.get::<SafeString>("field_name")?;
                let value = data.get::<DataStoreValue>("value")?;
                Filter::NotILikeCond {
                    field_name,
                    value: value.into(),
                }
            },
            "Group" => {
                let LuaValue::Table(data) = data else {
                    return Err(LuaError::external("Invalid filter data. Must be a table"));
                };
                let filters = data.get::<Vec<FilterWithContinuation>>("filters")?;
                Filter::Group {
                    filters,
                }
            },
            _ => {
                return Err(LuaError::external("Invalid filter type"));
            }
        };

        Ok(filter)
    }

    pub fn validate(&self, allowed_field_names: &[SafeString]) -> bool {
        match self {
            Filter::EqualCond { field_name, .. }
            | Filter::NotEqualCond { field_name, .. }
            | Filter::GreaterCond { field_name, .. }
            | Filter::GreaterEqualCond { field_name, .. }
            | Filter::LessCond { field_name, .. }
            | Filter::LessEqualCond { field_name, .. }
            | Filter::InCond { field_name, .. }
            | Filter::NotInCond { field_name, .. }
            | Filter::LikeCond { field_name, .. }
            | Filter::NotLikeCond { field_name, .. }
            | Filter::ILikeCond { field_name, .. } => {
                allowed_field_names.contains(field_name)
            } 
            | Filter::NotILikeCond { field_name, .. } => {
                allowed_field_names.contains(field_name)
            }
            Filter::Group { filters } => {
                filters.iter().all(|f| f.filter.validate(allowed_field_names))
            }
        }
    }

    /// Helper function to convert the filter to SQL
    pub fn to_sql(self, l: &mut Vec<Box<DataStoreValue>>) -> String {
        match self {
            Filter::EqualCond { field_name, value } => {
                if value.is_null() {
                    return format!("{} IS NULL", field_name);
                }

                l.push(value);
                format!("{} = ${}", field_name, l.len())
            }
            Filter::NotEqualCond { field_name, value } => {
                if value.is_null() {
                    return format!("{} IS NOT NULL", field_name);
                }

                l.push(value);
                format!("{} != ${}", field_name, l.len())
            }
            Filter::GreaterCond { field_name, value } => {
                l.push(value);
                format!("{} > ${}", field_name, l.len())
            }
            Filter::GreaterEqualCond { field_name, value } => {
                l.push(value);
                format!("{} >= ${}", field_name, l.len())
            }
            Filter::LessCond { field_name, value } => {
                l.push(value);
                format!("{} < ${}", field_name, l.len())
            }
            Filter::LessEqualCond { field_name, value } => {
                l.push(value);
                format!("{} <= ${}", field_name, l.len())
            }
            Filter::InCond { field_name, values } => {
                if values.is_empty() {
                    return "(1 = 1)".to_string(); // No filters, return true
                }

                let mut sql = format!("{} IN (", field_name);
                let v_len = values.len();
                for (i, value) in values.into_iter().enumerate() {
                    l.push(value);
                    if i == v_len - 1 {
                        sql.push_str(&format!("${}", l.len()));
                    } else {
                        sql.push_str(&format!("${}, ", l.len()));
                    }
                }
                sql.push(')');
                sql
            }
            Filter::NotInCond { field_name, values } => {
                if values.is_empty() {
                    return "(1 = 1)".to_string(); // No filters, return true
                }

                let mut sql = format!("{} NOT IN (", field_name);
                let v_len = values.len();
                for (i, value) in values.into_iter().enumerate() {
                    l.push(value);
                    if i == v_len - 1 {
                        sql.push_str(&format!("${}", l.len()));
                    } else {
                        sql.push_str(&format!("${}, ", l.len()));
                    }
                }
                sql.push(')');
                sql
            }
            Filter::LikeCond { field_name, value } => {
                l.push(value);
                format!("{} LIKE ${}", field_name, l.len())
            }
            Filter::NotLikeCond { field_name, value } => {
                l.push(value);
                format!("{} NOT LIKE ${}", field_name, l.len())
            }
            Filter::ILikeCond { field_name, value } => {
                l.push(value);
                format!("{} ILIKE ${}", field_name, l.len())
            }
            Filter::NotILikeCond { field_name, value } => {
                l.push(value);
                format!("{} NOT ILIKE ${}", field_name, l.len())
            }
            Filter::Group { filters } => {
                if filters.is_empty() {
                    return "(1 = 1)".to_string(); // No filters, return true
                }

                let mut sql = "(".to_string();
                let filters_len = filters.len();
                for (i, filter) in filters.into_iter().enumerate() {
                    let filter_sql = filter.filter.to_sql(l);
                    let continuation = filter.continuation.unwrap_or(FilterContinuation::And);

                    if i == filters_len - 1 {
                        sql.push_str(&format!("{} ", filter_sql));
                    } else {
                        sql.push_str(&format!("{} {} ", filter_sql, continuation.to_sql()));
                    }
                }
                sql.push_str(")");
                sql
            }
        }
    } 
}

#[derive(Clone, Debug)]
pub enum FilterContinuation {
    And,
    Or,
}

impl FromLua for FilterContinuation {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let s = String::from_lua(value, lua)?;
        match s.as_str() {
            "And" => Ok(FilterContinuation::And),
            "Or" => Ok(FilterContinuation::Or),
            _ => Err(LuaError::FromLuaConversionError { from: "any", to: "FilterContinuation".to_string(), message: Some("Invalid filter continuation. Must be either `And` or `Or`".to_string()) }),
        }
    }
}

impl IntoLua for FilterContinuation {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let s = match self {
            FilterContinuation::And => "And",
            FilterContinuation::Or => "Or",
        };
        String::into_lua(s.to_string(), lua)
    }
}

impl FilterContinuation {
    pub fn to_sql(&self) -> &'static str {
        match self {
            FilterContinuation::And => "AND",
            FilterContinuation::Or => "OR",
        }
    }
}

#[derive(Clone, Debug)]
pub struct FilterWithContinuation {
    filter: Filter,
    continuation: Option<FilterContinuation>, // note: the last filter's continuation will be ignored
}

impl FromLua for FilterWithContinuation {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        let LuaValue::Table(table) = value else {
            return Err(LuaError::FromLuaConversionError { from: "any", to: "FilterWithContinuation".to_string(), message: Some("Expected a table".to_string()) });
        };

        let filter = table.get::<Filter>("filter")?;
        let continuation = table.get::<Option<FilterContinuation>>("continuation")?;

        Ok(Self {
            filter,
            continuation,
        })
    }
}

impl IntoLua for FilterWithContinuation {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("filter", self.filter)?;
        table.set("continuation", self.continuation)?;
        Ok(LuaValue::Table(table))
    }
}

#[derive(Clone, Debug)]
pub struct Filters {
    filters: Vec<FilterWithContinuation>,
}

impl FromLua for Filters {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        let LuaValue::Table(table) = value else {
            return Err(LuaError::FromLuaConversionError { from: "any", to: "Filters".to_string(), message: Some("Expected a table".to_string()) });
        };

        let filters = table.get::<Vec<FilterWithContinuation>>("filters")?;

        Ok(Self { filters })
    }
}

impl IntoLua for Filters {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        let filters_table = lua.create_table()?;
        for (i, filter) in self.filters.into_iter().enumerate() {
            let filter_lua = filter.into_lua(lua)?;
            filters_table.set(i + 1, filter_lua)?;
        }
        table.set("filters", filters_table)?;
        Ok(LuaValue::Table(table))
    }
}

impl Filters {
    pub fn validate(&self, allowed_field_names: &[SafeString]) -> bool {
        self.filters.iter().all(|f| f.filter.validate(allowed_field_names))
    }

    pub fn to_sql(self, allowed_field_names: &[SafeString]) -> (String, Vec<Box<DataStoreValue>>) {
        let mut sql = String::new();
        let mut values = Vec::new();

        let filters_len = self.filters.len();
        for (i, filter) in self.filters.into_iter().enumerate() {
            if !filter.filter.validate(allowed_field_names) {
                continue;
            }
            let filter_sql = filter.filter.to_sql(&mut values);
            let continuation = filter.continuation.unwrap_or(FilterContinuation::And);

            if i == filters_len - 1 {
                sql.push_str(&filter_sql);
            } else {
                sql.push_str(&format!("{} {} ", filter_sql, continuation.to_sql()));
            }
        }

        if sql.is_empty() {
            ("(1 = 1)".to_string(), Vec::with_capacity(0)) // No filters, return true
        } else {
            (sql, values)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DataStoreTypeModifier {
    Scalar,
    Array,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DataStoreColumnType {
    Text,
    Integer,
    Float,
    Boolean,
    Json,
    Timestamptz,
    Interval,
    TimeZone,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DataStoreColumn {
    pub name: SafeString,
    pub type_modifier: DataStoreTypeModifier,
    pub column_type: DataStoreColumnType,
    pub primary_key: bool,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub enum DataStoreValue {
    Text(String),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    Boolean(bool),
    Map(indexmap::IndexMap<String, DataStoreValue>),
    List(Vec<DataStoreValue>),
    Timestamptz(chrono::DateTime<chrono::Utc>),
    Interval(chrono::Duration),
    TimeZone(chrono_tz::Tz),
    Filter(Filter),
    FilterContinuation(FilterWithContinuation),
    FilterWithContinuation(FilterWithContinuation),
    Filters(Filters),
    Null,
}

impl FromLua for Box<DataStoreValue> {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let value = DataStoreValue::from_lua_impl(value, lua, 0)?;
        Ok(Box::new(value))
    }
}

impl IntoLua for Box<DataStoreValue> {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        (*self).into_lua_impl(lua, 0)
    }
}

impl From<String> for DataStoreValue {
    fn from(value: String) -> Self {
        DataStoreValue::Text(value)
    }
}
impl From<i64> for DataStoreValue {
    fn from(value: i64) -> Self {
        DataStoreValue::Integer(value)
    }
}
impl From<u64> for DataStoreValue {
    fn from(value: u64) -> Self {
        DataStoreValue::UnsignedInteger(value)
    }
}
impl From<f64> for DataStoreValue {
    fn from(value: f64) -> Self {
        DataStoreValue::Float(value)
    }
}
impl From<bool> for DataStoreValue {
    fn from(value: bool) -> Self {
        DataStoreValue::Boolean(value)
    }
}
impl From<chrono::DateTime<chrono::Utc>> for DataStoreValue {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        DataStoreValue::Timestamptz(value)
    }
}
impl From<chrono::Duration> for DataStoreValue {
    fn from(value: chrono::Duration) -> Self {
        DataStoreValue::Interval(value)
    }
}
impl From<chrono_tz::Tz> for DataStoreValue {
    fn from(value: chrono_tz::Tz) -> Self {
        DataStoreValue::TimeZone(value)
    }
}

impl From<Filter> for DataStoreValue {
    fn from(value: Filter) -> Self {
        DataStoreValue::Filter(value)
    }
}

impl From<FilterWithContinuation> for Box<DataStoreValue> {
    fn from(value: FilterWithContinuation) -> Self {
        DataStoreValue::FilterWithContinuation(value).into()
    }
}

impl From<FilterWithContinuation> for DataStoreValue {
    fn from(value: FilterWithContinuation) -> Self {
        DataStoreValue::FilterWithContinuation(value)
    }
}

impl From<Filters> for DataStoreValue {
    fn from(value: Filters) -> Self {
        DataStoreValue::Filters(value)
    }
}

impl From<()> for DataStoreValue {
    fn from(_: ()) -> Self {
        DataStoreValue::Null
    }
}


impl<T> From<Option<T>> for DataStoreValue
where
    T: Into<DataStoreValue>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => DataStoreValue::Null,
        }
    }
}

impl<T> From<Vec<T>> for DataStoreValue
where
    T: Into<DataStoreValue>,
{
    fn from(value: Vec<T>) -> Self {
        DataStoreValue::List(value.into_iter().map(|v| v.into()).collect())
    }
}

impl<T> From<indexmap::IndexMap<String, T>> for DataStoreValue
where
    T: Into<DataStoreValue>,
{
    fn from(value: indexmap::IndexMap<String, T>) -> Self {
        DataStoreValue::Map(value.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

/// Macro to cheaply create a DataStoreValue
///
/// data_store_value!(1, 2, 3) will create a DataStoreValue::List(vec![
///     DataStoreValue::Integer(1),
///     DataStoreValue::Integer(2),
///     DataStoreValue::Integer(3),
/// ]);
///
/// and data_store_value!(1) will create a DataStoreValue::Integer(1)
/// and data_store_value!("hello" => "world") will create a DataStoreValue::Map(indexmap!{"hello".to_string() => DataStoreValue::Text("world".to_string())})
#[macro_export]
macro_rules! data_store_value {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = indexmap::IndexMap::new();
            $(
                map.insert($key.to_string(), $value);
            )*
            DataStoreValue::Map(map)
        }
    };
    ($($value:expr),*) => {
        {
            let mut list = Vec::new();
            $(
                list.push($value);
            )*
            DataStoreValue::List(list)
        }
    };
    ($value:expr) => {
        ($value).into()
    };
}

impl DataStoreValue {
    pub fn is_null(&self) -> bool {
        matches!(self, DataStoreValue::Null)
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            DataStoreValue::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            DataStoreValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_unsigned_integer(&self) -> Option<u64> {
        match self {
            DataStoreValue::UnsignedInteger(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            DataStoreValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            DataStoreValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&indexmap::IndexMap<String, DataStoreValue>> {
        match self {
            DataStoreValue::Map(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<DataStoreValue>> {
        match self {
            DataStoreValue::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_timestamptz(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        match self {
            DataStoreValue::Timestamptz(dt) => Some(*dt),
            _ => None,
        }
    }

    pub fn as_interval(&self) -> Option<chrono::Duration> {
        match self {
            DataStoreValue::Interval(dt) => Some(*dt),
            _ => None,
        }
    }

    pub fn as_timezone(&self) -> Option<chrono_tz::Tz> {
        match self {
            DataStoreValue::TimeZone(tz) => Some(tz.clone()),
            _ => None,
        }
    }

    pub fn as_filter(&self) -> Option<&Filter> {
        match self {
            DataStoreValue::Filter(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_filter_continuation(&self) -> Option<&FilterWithContinuation> {
        match self {
            DataStoreValue::FilterContinuation(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_filter_with_continuation(&self) -> Option<&FilterWithContinuation> {
        match self {
            DataStoreValue::FilterWithContinuation(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_filters(&self) -> Option<&Filters> {
        match self {
            DataStoreValue::Filters(f) => Some(f),
            _ => None,
        }
    }

    fn from_lua_impl(value: LuaValue, lua: &Lua, depth: usize) -> LuaResult<Self> {
        if depth > 10 {
            return Err(LuaError::FromLuaConversionError { from: "any", to: "DataStoreValue".to_string(), message: Some("Recursion limit exceeded".to_string()) });
        }

        match value {
            LuaValue::String(s) => Ok(DataStoreValue::Text(s.to_string_lossy().to_string())),
            LuaValue::Integer(i) => Ok(DataStoreValue::Integer(i)),
            LuaValue::Number(f) => Ok(DataStoreValue::Float(f)),
            LuaValue::Boolean(b) => Ok(DataStoreValue::Boolean(b)),
            LuaValue::Nil => Ok(DataStoreValue::Null),
            LuaValue::Table(table) => {
                if table.raw_len() == 0 {
                    // Map
                    let mut map = indexmap::IndexMap::new();
                    for pair in table.pairs::<String, LuaValue>() {
                        let (k, v) = pair?;
                        let v = DataStoreValue::from_lua_impl(v, lua, depth+1)?;
                        map.insert(k, v);
                    }
                    return Ok(DataStoreValue::Map(map));
                }
                // Check if the table is a list
                let mut list = Vec::new();
                for v in table.sequence_values::<LuaValue>() {
                    let v = v?;
                    let v = DataStoreValue::from_lua_impl(v, lua, depth+1)?;
                    list.push(v);
                }

                Ok(DataStoreValue::List(list))
            }
            LuaValue::UserData(ud) => {
                if let Ok(dt) = ud.borrow::<crate::plugins::antiraid::datetime::DateTime<chrono_tz::Tz>>() {
                    return Ok(DataStoreValue::Timestamptz(dt.dt.with_timezone(&chrono::Utc)));
                }
                if let Ok(delta) = ud.borrow::<crate::plugins::antiraid::datetime::TimeDelta>() {
                    return Ok(DataStoreValue::Interval(delta.timedelta));
                }
                if let Ok(tz) = ud.borrow::<crate::plugins::antiraid::datetime::Timezone>() {
                    return Ok(DataStoreValue::TimeZone(tz.tz.clone()));
                }
                if let Ok(i_64) = ud.borrow::<crate::plugins::antiraid::typesext::I64>() {
                    return Ok(DataStoreValue::Integer(i_64.0));
                }
                if let Ok(u_64) = ud.borrow::<crate::plugins::antiraid::typesext::U64>() {
                    return Ok(DataStoreValue::UnsignedInteger(u_64.0));
                }
                if let Ok(wrapped_datastore_type) = ud.borrow::<crate::plugins::antiraid::datastores::NamedDataStoreType>() {
                    match wrapped_datastore_type.name.as_str() {
                        "Filter" => {
                            let filter = Filter::from_lua(wrapped_datastore_type.data.clone(), lua)?;
                            return Ok(DataStoreValue::Filter(filter));
                        },
                        "FilterContinuation" => {
                            let filter = FilterWithContinuation::from_lua(wrapped_datastore_type.data.clone(), lua)?;
                            return Ok(DataStoreValue::FilterContinuation(filter));
                        },
                        "FilterWithContinuation" => {
                            let filter = FilterWithContinuation::from_lua(wrapped_datastore_type.data.clone(), lua)?;
                            return Ok(DataStoreValue::FilterWithContinuation(filter));
                        },
                        "Filters" => {
                            let filters = Filters::from_lua(wrapped_datastore_type.data.clone(), lua)?;
                            return Ok(DataStoreValue::Filters(filters));
                        },
                        _ => {
                            return Err(LuaError::FromLuaConversionError { from: "NamedDataStoreType", to: "DataStoreValue".to_string(), message: Some("Unknown NamedDataStoreType name".to_string()) });
                        }
                    }
                }

                return Err(LuaError::FromLuaConversionError { from: "userdata", to: "DateTime | TimeDelta | TimeZone".to_string(), message: Some("Invalid UserData type. Only DateTime, TimeDelta and NamedDataStoreType is supported at this time".to_string()) });
            }
            _ => Err(LuaError::FromLuaConversionError { from: "any", to: "DataStoreValue".to_string(), message: Some("Invalid type".to_string()) }),
        }
    }

    fn into_lua_impl(self, lua: &Lua, depth: usize) -> LuaResult<LuaValue> {
        if depth > 10 {
            return Err(LuaError::FromLuaConversionError { from: "any", to: "DataStoreValue".to_string(), message: Some("Recursion limit exceeded".to_string()) });
        }

        match self {
            DataStoreValue::Text(s) => Ok(LuaValue::String(lua.create_string(&s)?)),
            DataStoreValue::Integer(i) => {
                // If i is above/below the 52 bit precision limit, use a typesext.I64
                let min_luau_integer = -9007199254740991; // 2^53 - 1
                let max_luau_integer = 9007199254740991; // 2^53 - 1
                if i > max_luau_integer || i < min_luau_integer {
                    crate::plugins::antiraid::typesext::I64(i).into_lua(lua)
                } else {
                    Ok(LuaValue::Integer(i))
                }
            },
            DataStoreValue::UnsignedInteger(i) => crate::plugins::antiraid::typesext::U64(i).into_lua(lua), // An UnsignedInteger can only be created through explicit U64 parse
            DataStoreValue::Float(f) => Ok(LuaValue::Number(f)),
            DataStoreValue::Boolean(b) => Ok(LuaValue::Boolean(b)),
            DataStoreValue::Map(j) => {
                let table = lua.create_table()?;
                for (k, v) in j.into_iter() {
                    let v = v.into_lua_impl(lua, depth+1)?;
                    table.set(k, v)?;
                }
                Ok(LuaValue::Table(table))
            }
            DataStoreValue::List(l) => {
                let table = lua.create_table()?;
                for v in l.into_iter() {
                    let v = v.into_lua_impl(lua, depth+1)?;
                    table.set(table.raw_len() + 1, v)?;
                }
                Ok(LuaValue::Table(table))
            }
            DataStoreValue::Timestamptz(dt) => crate::plugins::antiraid::datetime::DateTime::<chrono_tz::Tz>::from_utc(dt).into_lua(lua),
            DataStoreValue::Interval(i) => crate::plugins::antiraid::datetime::TimeDelta::new(i).into_lua(lua),
            DataStoreValue::TimeZone(tz) => crate::plugins::antiraid::datetime::Timezone::new(tz).into_lua(lua),
            DataStoreValue::Filter(filter) => {
                let value = filter.into_lua(lua)?;
                crate::plugins::antiraid::datastores::NamedDataStoreType::new("Filter".to_string(), value).into_lua(lua)
            }
            DataStoreValue::FilterContinuation(filter) => {
                let value = filter.into_lua(lua)?;
                crate::plugins::antiraid::datastores::NamedDataStoreType::new("FilterContinuation".to_string(), value).into_lua(lua)
            }
            DataStoreValue::FilterWithContinuation(filter) => {
                let value = filter.into_lua(lua)?;
                crate::plugins::antiraid::datastores::NamedDataStoreType::new("FilterWithContinuation".to_string(), value).into_lua(lua)
            }
            DataStoreValue::Filters(filters) => {
                let value = filters.into_lua(lua)?;
                crate::plugins::antiraid::datastores::NamedDataStoreType::new("Filters".to_string(), value).into_lua(lua)
            }
            DataStoreValue::Null => Ok(LuaValue::Nil),
        }
    }

    pub fn validate(&self, columns: &[DataStoreColumn]) -> Vec<String> {
        fn validate_inner(errors: &mut Vec<String>, column: &DataStoreColumn, data: &DataStoreValue) {
            match column.column_type {
                DataStoreColumnType::Text => {
                    match data {
                        DataStoreValue::Text(_) => return,
                        DataStoreValue::Null => {
                            if column.nullable {
                                return;
                            }
                            errors.push(format!("Column {} is not nullable", column.name));
                        },
                        _ => {
                            errors.push(format!("Column {} is not a string", column.name));
                        }
                    }
                }
                DataStoreColumnType::Integer => {
                    match data {
                        DataStoreValue::Integer(_) => return,
                        DataStoreValue::UnsignedInteger(_) => return,
                        DataStoreValue::Null => {
                            if column.nullable {
                                return;
                            }
                            errors.push(format!("Column {} is not nullable", column.name));
                        },
                        _ => {
                            errors.push(format!("Column {} is not an integer", column.name));
                        }
                    }
                }
                DataStoreColumnType::Float => {
                    match data {
                        DataStoreValue::Float(_) => return,
                        DataStoreValue::Null => {
                            if column.nullable {
                                return;
                            }
                            errors.push(format!("Column {} is not nullable", column.name));
                        },
                        _ => {
                            errors.push(format!("Column {} is not a float", column.name));
                        }
                    }
                }
                DataStoreColumnType::Boolean => {
                    match data {
                        DataStoreValue::Boolean(_) => return,
                        DataStoreValue::Null => {
                            if column.nullable {
                                return;
                            }
                            errors.push(format!("Column {} is not nullable", column.name));
                        },
                        _ => {
                            errors.push(format!("Column {} is not a boolean", column.name));
                        }
                    }
                }
                DataStoreColumnType::Json => {
                    match data {
                        DataStoreValue::Null => {
                            if column.nullable {
                                return;
                            }
                            errors.push(format!("Column {} is not nullable", column.name));
                        },
                        _ => {} // JSON is valid if it's not null
                    }
                }
                DataStoreColumnType::Timestamptz => {
                    match data {
                        DataStoreValue::Timestamptz(_) => return,
                        DataStoreValue::Null => {
                            if column.nullable {
                                return;
                            }
                            errors.push(format!("Column {} is not nullable", column.name));
                        },
                        _ => {
                            errors.push(format!("Column {} is not a timestamp", column.name));
                        }
                    }
                }
                DataStoreColumnType::Interval => {
                    match data {
                        DataStoreValue::Interval(_) => return,
                        DataStoreValue::Null => {
                            if column.nullable {
                                return;
                            }
                            errors.push(format!("Column {} is not nullable", column.name));
                        },
                        _ => {
                            errors.push(format!("Column {} is not an interval", column.name));
                        }
                    }
                },
                DataStoreColumnType::TimeZone => {
                    match data {
                        DataStoreValue::TimeZone(_) => return,
                        DataStoreValue::Null => {
                            if column.nullable {
                                return;
                            }
                            errors.push(format!("Column {} is not nullable", column.name));
                        }
                        _ => {
                            errors.push(format!("Column {} is not a timezone", column.name));
                        }
                    }
                }
            }
        }

        let Some(map) = self.as_map() else {
            return vec!["Data is not a map".to_string()];
        };

        let mut errors = Vec::new();
        for column in columns.iter() {
            let Some(v) = map.get(&column.name.to_string()) else {
                if !column.nullable {
                    errors.push(format!("Column {} is not nullable", column.name));
                }
                continue;
            };

            match column.type_modifier {
                DataStoreTypeModifier::Scalar => {
                    validate_inner(&mut errors, column, v);
                }
                DataStoreTypeModifier::Array => {
                    match v {
                        DataStoreValue::List(l) => {
                            for item in l.iter() {
                                validate_inner(&mut errors, column, item);
                            }
                        }
                        _ => {
                            errors.push(format!("Column {} is not an array", column.name));
                        }
                    }
                }
            }
        }

        errors
    }
}

impl FromLua for DataStoreValue {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        DataStoreValue::from_lua_impl(value, lua, 0)
    }
}

impl IntoLua for DataStoreValue {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        DataStoreValue::into_lua_impl(self, lua, 0)
    }
}

#[derive(Clone)]
pub struct ValidateColumnsAgainstData {
    pub errors: Vec<String>,
    pub parsed_data: DataStoreValue,
}

impl IntoLua for ValidateColumnsAgainstData {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("errors", self.errors)?;
        table.set("parsed_data", self.parsed_data)?;
        Ok(LuaValue::Table(table))
    }
}

pub type DataStoreMethodResult = Pin<Box<dyn Future<Output = Result<DataStoreValue, crate::Error>>>>;
pub type DataStoreMethod = Rc<dyn Fn(Vec<DataStoreValue>) -> DataStoreMethodResult>;

#[async_trait(?Send)]
pub trait DataStoreImpl {
    fn name(&self) -> String;
    fn table_name(&self) -> SafeString;
    fn needs_caps(&self) -> bool;
    fn columns(&self) -> Vec<DataStoreColumn>;

    fn column_names(&self) -> Vec<SafeString> {
        self.columns()
            .iter()
            .map(|c| c.name.clone())
            .collect()
    }

    /// Debug method to get a corresponding SQL string for the filters
    ///
    /// Note: there is no guarantee that the datastore will use this SQL string or
    /// that the datastore will even be SQL based
    fn filters_sql(&self, filters: Filters) -> (String, Vec<Box<DataStoreValue>>) {
        filters.to_sql(&self.column_names())
    }

    /// Validate the data against the columns returning the validated data
    fn validate_data_against_columns(&self, lua: &Lua, data: &LuaValue) -> ValidateColumnsAgainstData {
        validate_data_against_columns(&self.columns(), lua, data)
    }

    /// Returns a list of methods
    fn methods(&self) -> Vec<String>;

    /// Gets a method
    fn get_method(&self, key: String) -> Option<DataStoreMethod>;
}

pub fn validate_data_against_columns(columns: &[DataStoreColumn], lua: &Lua, data: &LuaValue) -> ValidateColumnsAgainstData {
    let parsed_data = match DataStoreValue::from_lua(data.clone(), lua) {
        Ok(parsed_data) => parsed_data,
        Err(e) => {
            return ValidateColumnsAgainstData {
                errors: vec![format!("Failed to parse data: {}", e)],
                parsed_data: DataStoreValue::Null,
            };
        }
    };

    ValidateColumnsAgainstData {
        errors: parsed_data.validate(columns),
        parsed_data,
    }
}

/// A data store to copy whatever is input to copy()
pub struct CopyDataStore;

#[async_trait(?Send)]
impl DataStoreImpl for CopyDataStore {
    fn name(&self) -> String {
        "CopyDataStore".to_string()
    }

    fn table_name(&self) -> SafeString {
        SafeString::new("".to_string()).unwrap()
    }

    fn needs_caps(&self) -> bool {
        false
    }

    fn columns(&self) -> Vec<DataStoreColumn> {
        vec![
            /*DataStoreColumn {
                name: SafeString::new("id".to_string()).unwrap(),
                type_modifier: DataStoreTypeModifier::Scalar,
                column_type: DataStoreColumnType::Integer,
                primary_key: true,
                nullable: false,
            },
            DataStoreColumn {
                name: SafeString::new("name".to_string()).unwrap(),
                type_modifier: DataStoreTypeModifier::Scalar,
                column_type: DataStoreColumnType::Text,
                primary_key: false,
                nullable: false,
            },*/
        ]
    }

    fn methods(&self) -> Vec<String> {
        vec!["copy".to_string()]
    }

    fn get_method(&self, key: String) -> Option<DataStoreMethod> {
        if key == "copy" {
            Some(Rc::new(|v| {
                Box::pin(async { 
                    let mut v = v;
                    if v.len() == 0 {
                        return Ok(DataStoreValue::Null);
                    } else if v.len() == 1 {
                        let Some(v) = v.pop() else {
                            return Ok(DataStoreValue::Null);
                        };
                        return Ok(v);
                    } else {
                        return Ok(DataStoreValue::List(v));
                    }
                })
            }))
        } else {
            None
        }
    }
}