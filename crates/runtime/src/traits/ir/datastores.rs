use mlua::prelude::*;
use serenity::async_trait;
use std::collections::HashMap;

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
    Json(serde_json::Value),
    List(Vec<DataStoreValue>),
    Timestamptz(chrono::DateTime<chrono::Utc>),
    Interval(chrono::Duration),
    Null,
}

impl IntoLua for DataStoreValue {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let mut stack = std::collections::VecDeque::new();
        stack.push_back((self, None));

        let root_table = lua.create_table()?;
        loop {
            let Some((el, parent)) = stack.pop_front() else {
                break;
            };

            let (_typ, dsv) = match el {
                DataStoreValue::Text(s) => ("Text", LuaValue::String(lua.create_string(&s)?)),
                DataStoreValue::Integer(i) => ("Integer", LuaValue::Integer(i)),
                DataStoreValue::Float(f) => ("Float", LuaValue::Number(f)),
                DataStoreValue::Boolean(b) => ("Boolean", LuaValue::Boolean(b)),
                DataStoreValue::Json(j) => ("Json", lua.to_value(&j)?),
                DataStoreValue::List(l) => {
                    let table = lua.create_table()?;
                    for v in l.into_iter() {
                        stack.push_front((v, Some(table.clone())));
                    }
                    ("List", LuaValue::Table(table))
                }
                DataStoreValue::Timestamptz(dt) => ("Timestamptz", crate::plugins::antiraid::datetime::DateTime::<chrono_tz::Tz>::from_utc(dt).into_lua(lua)?),
                DataStoreValue::Interval(i) => ("Interval", crate::plugins::antiraid::datetime::TimeDelta::new(i).into_lua(lua)?),
                DataStoreValue::Null => ("Null", LuaValue::Nil),
            };

            //let tab = lua.create_table()?;
            //tab.set("type", typ.to_string())?;
            //tab.set("value", dsv)?;

            match parent {
                Some(parent) => {
                    parent.set(parent.raw_len() + 1, dsv)?;
                }
                None => {
                    root_table.set(root_table.raw_len() + 1, dsv)?;
                }
            }
        }

        Ok(LuaValue::Table(root_table))
    }
}

#[derive(Clone)]
/// A map of string to DataStoreValue
pub struct DataStoreValueMap(pub HashMap<String, DataStoreValue>);

impl IntoLua for DataStoreValueMap {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        for (k, v) in self.0.into_iter() {
            let v = v.into_lua(lua)?;
            table.set(k, v)?;
        }
        Ok(LuaValue::Table(table))
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

    async fn list(&self) -> Result<Vec<DataStoreValueMap>, crate::Error>;
    async fn get(&self, filters: Filters) -> Result<DataStoreValueMap, crate::Error>;
    async fn insert(&self, data: DataStoreValueMap) -> Result<DataStoreValueMap, crate::Error>;
    async fn update(&self, filters: Filters, data: DataStoreValueMap) -> Result<DataStoreValueMap, crate::Error>;
    async fn delete(&self, filters: Filters) -> Result<(), crate::Error>;
    async fn count(&self, filters: Filters) -> Result<i64, crate::Error>;
}

pub fn validate_data_against_columns(columns: &[DataStoreColumn], lua: &Lua, data: &LuaValue) -> ValidateColumnsAgainstData {
    fn parse_column(lua: &Lua, column: &DataStoreColumn, v: LuaValue) -> Result<DataStoreValue, String> {
        match column.column_type {
            DataStoreColumnType::Text => {
                let Some(s) = v.as_string_lossy() else {
                    return Err(format!("Column {} is not a string", column.name));
                };
                
                Ok(DataStoreValue::Text(s))
            }
            DataStoreColumnType::Integer => {
                let Some(v) = v.as_i64() else {
                    return Err(format!("Column {} is not an integer", column.name));
                };
                
                Ok(DataStoreValue::Integer(v))
            }
            DataStoreColumnType::Float => {
                let Some(v) = v.as_f64() else {
                    return Err(format!("Column {} is not a float", column.name));
                };
                
                Ok(DataStoreValue::Float(v))
            }
            DataStoreColumnType::Boolean => {
                let Some(v) = v.as_boolean() else {
                    return Err(format!("Column {} is not a boolean", column.name));
                };
                Ok(DataStoreValue::Boolean(v))
            }
            DataStoreColumnType::Json => {
                let Ok(v) = lua.from_value::<serde_json::Value>(v) else {
                    return Err(format!("Column {} is not a valid JSON", column.name));
                };
                
                Ok(DataStoreValue::Json(v))
            }
            DataStoreColumnType::Timestamptz => {
                match v {
                    LuaValue::String(ref s) => {
                        let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&s.to_string_lossy()) else {
                            return Err(format!("Column {} is not a valid UTC DateTime string", column.name));
                        };

                        let dt = dt.with_timezone(&chrono::Utc);

                        Ok(DataStoreValue::Timestamptz(dt))
                    },
                    LuaValue::UserData(ud) => {
                        let Ok(dt) = ud.borrow::<crate::plugins::antiraid::datetime::DateTime<chrono_tz::Tz>>() else {
                            return Err(format!("Column {} is not a valid UTC DateTime object", column.name));
                        };

                        let dt: chrono::DateTime<chrono::Utc> = dt.dt.with_timezone(&chrono::Utc);

                        Ok(DataStoreValue::Timestamptz(dt))
                    }
                    _ => {
                        Err(format!("Column {} is not a timestamp", column.name))
                    }
                }
            }
            DataStoreColumnType::Interval => {
                match v {
                    LuaValue::String(ref s) => {
                        // Parse string to number of seconds
                        let Ok(nsecs) = s.to_string_lossy().parse::<i64>() else {
                            return Err(format!("Column {} is not a valid number of seconds [interval type]", column.name));
                        };

                        Ok(DataStoreValue::Interval(chrono::Duration::seconds(nsecs)))
                    },
                    LuaValue::UserData(ud) => {
                        let Ok(delta) = ud.borrow::<crate::plugins::antiraid::datetime::TimeDelta>() else {
                            return Err(format!("Column {} is not a valid interval type", column.name));
                        };

                        Ok(DataStoreValue::Interval(delta.timedelta))
                    }
                    _ => {
                        return Err(format!("Column {} is not a valid interval type", column.name));
                    }
                }
            }
        }
    }

    let data = match data {
        LuaValue::Table(ref table) => {
            table       
        }
        _ => {
            return ValidateColumnsAgainstData {
                errors: vec!["Data is not a table".to_string()],
                parsed_data: DataStoreValueMap(HashMap::new()),
            };
        }
    };

    let mut errors = Vec::new();
    let mut parsed_data = HashMap::new();

    for column in columns.iter() {
        let v = match data.get::<LuaValue>(column.name.to_string()) {
            Ok(v) => v,
            Err(e) => {
                if column.nullable {
                    errors.push(format!("Error getting column {}: {}", column.name, e));
                    continue;
                } else {
                    errors.push(format!("Column {} is not nullable and received error: {}", column.name, e));
                    continue;
                }
            }
        };

        if v.is_nil() || v.is_null() {
            if !column.nullable {
                errors.push(format!("Column {} is not nullable", column.name));
            }

            continue;
        }

        match column.type_modifier {
            DataStoreTypeModifier::Scalar => {
                match parse_column(lua, column, v) {
                    Ok(parsed_value) => {
                        parsed_data.insert(column.name.to_string(), parsed_value);
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }        
            }
            DataStoreTypeModifier::Array => {
                let Some(v) = v.as_table() else {
                    errors.push(format!("Column {} is not an array", column.name));
                    continue;
                };
                let mut parsed_array = Vec::new();
                for v in v.sequence_values::<LuaValue>() {
                    let v = match v {
                        Ok(v) => v,
                        Err(e) => {
                            errors.push(format!("Error getting array value for column {}: {}", column.name, e));
                            continue;
                        }
                    };

                    match parse_column(lua, column, v) {
                        Ok(parsed_value) => {
                            parsed_array.push(parsed_value);
                        }
                        Err(e) => {
                            errors.push(e);
                        }
                    };
                }

                parsed_data.insert(column.name.to_string(), DataStoreValue::List(parsed_array));
            }
        }
    }

    ValidateColumnsAgainstData {
        errors,
        parsed_data: DataStoreValueMap(parsed_data),
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

    async fn list(&self) -> Result<Vec<DataStoreValueMap>, crate::Error> {
        Ok(vec![])
    }

    async fn get(&self, _filters: Filters) -> Result<DataStoreValueMap, crate::Error> {
        Ok(DataStoreValueMap(HashMap::new()))
    }

    async fn insert(&self, _data: DataStoreValueMap) -> Result<DataStoreValueMap, crate::Error> {
        Ok(DataStoreValueMap(HashMap::new()))
    }

    async fn update(&self, _filters: Filters, _data: DataStoreValueMap) -> Result<DataStoreValueMap, crate::Error> {
        Ok(DataStoreValueMap(HashMap::new()))
    }

    async fn delete(&self, _filters: Filters) -> Result<(), crate::Error> {
        Ok(())
    }

    async fn count(&self, _filters: Filters) -> Result<i64, crate::Error> {
        Ok(0)
    }
}