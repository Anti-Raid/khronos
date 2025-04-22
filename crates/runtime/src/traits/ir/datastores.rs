use mlua::prelude::*;
use serenity::async_trait;
use std::{future::Future, pin::Pin};
use std::rc::Rc;
use crate::utils::khronos_value::KhronosValue;

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
#[derive(Clone, Debug, serde::Serialize)]
pub enum Filter {
    EqualCond {
        field_name: SafeString,
        value: Box<KhronosValue>,
    },
    NotEqualCond {
        field_name: SafeString,
        value: Box<KhronosValue>,
    },
    GreaterCond {
        field_name: SafeString,
        value: Box<KhronosValue>,
    },
    GreaterEqualCond {
        field_name: SafeString,
        value: Box<KhronosValue>,
    },
    LessCond {
        field_name: SafeString,
        value: Box<KhronosValue>,
    },
    LessEqualCond {
        field_name: SafeString,
        value: Box<KhronosValue>,
    },
    InCond {
        field_name: SafeString,
        values: Vec<Box<KhronosValue>>,
    },
    NotInCond {
        field_name: SafeString,
        values: Vec<Box<KhronosValue>>,
    },
    LikeCond {
        field_name: SafeString,
        value: Box<KhronosValue>,
    },
    NotLikeCond {
        field_name: SafeString,
        value: Box<KhronosValue>,
    },
    ILikeCond {
        field_name: SafeString,
        value: Box<KhronosValue>,
    },
    NotILikeCond {
        field_name: SafeString,
        value: Box<KhronosValue>,
    },
    Group {
        filters: Vec<FilterWithContinuation>,
    }
}

impl Filter {
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
    pub fn to_sql(self, l: &mut Vec<Box<KhronosValue>>) -> String {
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum FilterContinuation {
    And,
    Or,
}

impl FilterContinuation {
    pub fn to_sql(&self) -> &'static str {
        match self {
            FilterContinuation::And => "AND",
            FilterContinuation::Or => "OR",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct FilterWithContinuation {
    filter: Filter,
    continuation: Option<FilterContinuation>, // note: the last filter's continuation will be ignored
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Filters {
    filters: Vec<FilterWithContinuation>,
}

impl Filters {
    #[allow(dead_code)]
    pub fn validate(&self, allowed_field_names: &[SafeString]) -> bool {
        self.filters.iter().all(|f| f.filter.validate(allowed_field_names))
    }

    #[allow(dead_code)]
    pub fn to_sql(self, allowed_field_names: &[SafeString]) -> (String, Vec<Box<KhronosValue>>) {
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

#[derive(Clone)]
pub struct ValidateColumnsAgainstData {
    pub errors: Vec<String>,
    pub parsed_data: KhronosValue,
}

impl IntoLua for ValidateColumnsAgainstData {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("errors", self.errors)?;
        table.set("parsed_data", self.parsed_data)?;
        Ok(LuaValue::Table(table))
    }
}

pub type DataStoreMethodResult = Pin<Box<dyn Future<Output = Result<KhronosValue, crate::Error>>>>;
pub type DataStoreMethod = Rc<dyn Fn(Vec<KhronosValue>) -> DataStoreMethodResult>;

#[async_trait(?Send)]
pub trait DataStoreImpl {
    fn name(&self) -> String;
    fn need_caps(&self, method: &str) -> bool;

    /// Returns a list of methods
    fn methods(&self) -> Vec<String>;

    /// Gets a method
    fn get_method(&self, key: String) -> Option<DataStoreMethod>;
}

fn validate_khronos_value(kv: &KhronosValue, columns: &[DataStoreColumn]) -> Vec<String> {
    fn validate_inner(errors: &mut Vec<String>, column: &DataStoreColumn, data: &KhronosValue) {
        match column.column_type {
            DataStoreColumnType::Text => {
                match data {
                    KhronosValue::Text(_) => return,
                    KhronosValue::Null => {
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
                    KhronosValue::Integer(_) => return,
                    KhronosValue::UnsignedInteger(_) => return,
                    KhronosValue::Null => {
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
                    KhronosValue::Float(_) => return,
                    KhronosValue::Null => {
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
                    KhronosValue::Boolean(_) => return,
                    KhronosValue::Null => {
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
                    KhronosValue::Null => {
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
                    KhronosValue::Timestamptz(_) => return,
                    KhronosValue::Null => {
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
                    KhronosValue::Interval(_) => return,
                    KhronosValue::Null => {
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
                    KhronosValue::TimeZone(_) => return,
                    KhronosValue::Null => {
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

    let Some(map) = kv.as_map() else {
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
                    KhronosValue::List(l) => {
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

pub fn validate_data_against_columns(columns: &[DataStoreColumn], lua: &Lua, data: &LuaValue) -> ValidateColumnsAgainstData {
    let parsed_data = match KhronosValue::from_lua(data.clone(), lua) {
        Ok(parsed_data) => parsed_data,
        Err(e) => {
            return ValidateColumnsAgainstData {
                errors: vec![format!("Failed to parse data: {}", e)],
                parsed_data: KhronosValue::Null,
            };
        }
    };

    ValidateColumnsAgainstData {
        errors: validate_khronos_value(&parsed_data, columns),
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

    fn need_caps(&self, _method: &str) -> bool {
        false
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
                        return Ok(KhronosValue::Null);
                    } else if v.len() == 1 {
                        let Some(v) = v.pop() else {
                            return Ok(KhronosValue::Null);
                        };
                        return Ok(v);
                    } else {
                        return Ok(KhronosValue::List(v));
                    }
                })
            }))
        } else {
            None
        }
    }
}