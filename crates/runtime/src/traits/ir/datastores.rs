use mlua::prelude::*;
use serenity::async_trait;

#[derive(Clone)]
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
    pub fn to_sql(self) -> (String, Vec<serde_json::Value>) {
        if self.filters.is_empty() {
            return ("(1 = 1)".to_string(), Vec::with_capacity(0)); // No filters, return true
        }

        let mut sql = String::new();
        let mut values = Vec::new();

        let filters_len = self.filters.len();
        for (i, filter) in self.filters.into_iter().enumerate() {
            let filter_sql = filter.filter.to_sql(&mut values);
            let continuation = filter.continuation.unwrap_or(FilterContinuation::And);

            if i == filters_len - 1 {
                sql.push_str(&format!("{} ", filter_sql));
            } else {
                sql.push_str(&format!("{} {} ", filter_sql, continuation.to_sql()));
            }
        }

        (sql, values)
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

#[async_trait(?Send)]
pub trait DataStoreImpl {
    fn name(&self) -> String;
    fn table_name(&self) -> String;
    fn columns(&self) -> Vec<DataStoreColumn>;
    async fn list(&self, lua: Lua) -> LuaResult<Vec<LuaValue>>;
    async fn get(&self, lua: Lua, filters: Filters) -> LuaResult<LuaValue>;
    async fn insert(&self, lua: Lua, data: LuaValue) -> LuaResult<LuaValue>;
    async fn update(&self, lua: Lua, filters: Filters, data: LuaValue) -> LuaResult<LuaValue>;
    async fn delete(&self, lua: Lua, filters: Filters) -> LuaResult<LuaValue>;
    async fn count(&self, lua: Lua, filters: Filters) -> LuaResult<LuaValue>;
}

pub struct DummyDataStoreImpl;

#[async_trait(?Send)]
impl DataStoreImpl for DummyDataStoreImpl {
    fn name(&self) -> String {
        "dummy".to_string()
    }

    fn table_name(&self) -> String {
        "dummy".to_string()
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

    async fn list(&self, _lua: Lua) -> LuaResult<Vec<LuaValue>> {
        Ok(vec![])
    }

    async fn get(&self, _lua: Lua, _filters: Filters) -> LuaResult<LuaValue> {
        Ok(LuaValue::Nil)
    }

    async fn insert(&self, _lua: Lua, _data: LuaValue) -> LuaResult<LuaValue> {
        Ok(LuaValue::Nil)
    }

    async fn update(&self, _lua: Lua, _filters: Filters, _data: LuaValue) -> LuaResult<LuaValue> {
        Ok(LuaValue::Nil)
    }

    async fn delete(&self, _lua: Lua, _filters: Filters) -> LuaResult<LuaValue> {
        Ok(LuaValue::Nil)
    }

    async fn count(&self, _lua: Lua, _filters: Filters) -> LuaResult<LuaValue> {
        Ok(LuaValue::Nil)
    }
}