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

/// The filters allowed for datastores
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Filter {
    EqualCond {
        field_name: SafeString,
        value: serde_json::Value,
    },
    NotEqualCond {
        field_name: SafeString,
        value: serde_json::Value,
    },
    GreaterCond {
        field_name: SafeString,
        value: serde_json::Value,
    },
    GreaterEqualCond {
        field_name: SafeString,
        value: serde_json::Value,
    },
    LessCond {
        field_name: SafeString,
        value: serde_json::Value,
    },
    LessEqualCond {
        field_name: SafeString,
        value: serde_json::Value,
    },
    InCond {
        field_name: SafeString,
        values: Vec<serde_json::Value>,
    },
    NotInCond {
        field_name: SafeString,
        values: Vec<serde_json::Value>,
    },
    LikeCond {
        field_name: SafeString,
        value: serde_json::Value,
    },
    ILikeCond {
        field_name: SafeString,
        value: serde_json::Value,
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
            | Filter::ILikeCond { field_name, .. } => {
                allowed_field_names.contains(field_name)
            }
            Filter::Group { filters } => {
                filters.iter().all(|f| f.filter.validate(allowed_field_names))
            }
        }
    }

    /// Helper function to convert the filter to SQL
    pub fn to_sql(self, l: &mut Vec<serde_json::Value>) -> String {
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
            Filter::ILikeCond { field_name, value } => {
                l.push(value);
                format!("{} ILIKE ${}", field_name, l.len())
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct FilterWithContinuation {
    filter: Filter,
    continuation: Option<FilterContinuation>, // note: the last filter's continuation will be ignored
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Filters {
    filters: Vec<FilterWithContinuation>,
}

impl Filters {
    pub fn validate(&self, allowed_field_names: &[SafeString]) -> bool {
        self.filters.iter().all(|f| f.filter.validate(allowed_field_names))
    }

    pub fn to_sql(self, allowed_field_names: &[SafeString]) -> (String, Vec<serde_json::Value>) {
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
    Interval
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
pub enum DataStoreValue {
    Text(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Map(indexmap::IndexMap<String, DataStoreValue>),
    List(Vec<DataStoreValue>),
    Timestamptz(chrono::DateTime<chrono::Utc>),
    Interval(chrono::Duration),
    Null,
}

impl DataStoreValue {
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
                return Err(LuaError::FromLuaConversionError { from: "userdata", to: "DateTime | TimeDelta".to_string(), message: Some("Invalid UserData type. Only DateTime and TimeDelta is supported at this time".to_string()) });
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
            DataStoreValue::Integer(i) => Ok(LuaValue::Integer(i)),
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
            DataStoreValue::Null => Ok(LuaValue::Nil),
        }
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
/// A map of string to DataStoreValue
pub struct DataStoreValueMap(pub indexmap::IndexMap<String, DataStoreValue>);

impl FromLua for DataStoreValueMap {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(table) => table,
            _ => return Err(LuaError::FromLuaConversionError { from: "any", to: "DataStoreValueMap".to_string(), message: Some("Expected a table".to_string()) }),
        };

        let mut map = indexmap::IndexMap::new();
        for pair in table.pairs::<String, LuaValue>() {
            let (k, v) = pair?;
            let v = DataStoreValue::from_lua(v, lua)?; // SAFETY: this is guaranteed to halt after depth 10
            map.insert(k, v);
        }

        Ok(DataStoreValueMap(map))
    }
}

impl IntoLua for DataStoreValueMap {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        for (k, v) in self.0.into_iter() {
            let v = v.into_lua(lua)?; // SAFETY: this is guaranteed to halt after depth 10
            table.set(k, v)?;
        }
        Ok(LuaValue::Table(table))
    }
}

impl DataStoreValueMap {
    pub fn new() -> Self {
        DataStoreValueMap(indexmap::IndexMap::new())
    }

    pub fn from_value(value: DataStoreValue) -> Option<Self> {
        match value {
            DataStoreValue::Map(map) => Some(DataStoreValueMap(map)),
            _ => None,
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
                }
            }
        }

        let mut errors = Vec::new();
        for column in columns.iter() {
            let Some(v) = self.0.get(&column.name.to_string()) else {
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

#[derive(Clone)]
pub struct ValidateColumnsAgainstData {
    pub errors: Vec<String>,
    pub parsed_data: DataStoreValueMap,
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

#[derive(Clone)]
pub enum DataStoreMethod {
    NoArgs(Rc<dyn Fn() -> DataStoreMethodResult>),
    Filters(Rc<dyn Fn(Filters) -> DataStoreMethodResult>),
    Value(Rc<dyn Fn(DataStoreValue) -> DataStoreMethodResult>),
    Map(Rc<dyn Fn(DataStoreValueMap) -> DataStoreMethodResult>),
    FiltersAndValue(Rc<dyn Fn(Filters, DataStoreValue) -> DataStoreMethodResult>),
    FiltersAndMap(Rc<dyn Fn(Filters, DataStoreValueMap) -> DataStoreMethodResult>),
    ValueAndMap(Rc<dyn Fn(DataStoreValue, DataStoreValueMap) -> DataStoreMethodResult>),
    FiltersAndValueAndMap(Rc<dyn Fn(Filters, DataStoreValue, DataStoreValueMap) -> DataStoreMethodResult>),
}

#[async_trait(?Send)]
pub trait DataStoreImpl {
    fn name(&self) -> String;
    fn table_name(&self) -> SafeString;
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
    fn filters_sql(&self, filters: Filters) -> (String, Vec<serde_json::Value>) {
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
    let parsed_data = match DataStoreValueMap::from_lua(data.clone(), lua) {
        Ok(parsed_data) => parsed_data,
        Err(e) => {
            return ValidateColumnsAgainstData {
                errors: vec![format!("Failed to parse data: {}", e)],
                parsed_data: DataStoreValueMap::new(),
            };
        }
    };

    ValidateColumnsAgainstData {
        errors: parsed_data.validate(columns),
        parsed_data,
    }
}

pub struct DummyDataStoreImpl;

#[async_trait(?Send)]
impl DataStoreImpl for DummyDataStoreImpl {
    fn name(&self) -> String {
        "dummy".to_string()
    }

    fn table_name(&self) -> SafeString {
        SafeString::new("dummy".to_string()).unwrap()
    }

    fn columns(&self) -> Vec<DataStoreColumn> {
        vec![
            DataStoreColumn {
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
            },
        ]
    }

    fn methods(&self) -> Vec<String> {
        vec!["returnValue".to_string()]
    }

    fn get_method(&self, key: String) -> Option<DataStoreMethod> {
        if key == "returnValue" {
            Some(DataStoreMethod::Value(Rc::new(|v| {
                Box::pin(async { Ok(v) })
            })))
        } else {
            None
        }
    }
}