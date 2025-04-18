use super::LUA_SERIALIZE_OPTIONS;
use mlua::prelude::*;
use serenity::async_trait;
use crate::lua_promise;
use std::rc::Rc;
use crate::primitives::create_userdata_iterator_with_fields;
use std::cell::RefCell;
use crate::traits::context::KhronosContext;
use crate::traits::datastoreprovider::DataStoreProvider;
use crate::TemplateContextRef;
use crate::utils::executorscope::ExecutorScope;

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
                l.push(value);
                format!("{} = ${}", field_name, l.len())
            }
            Filter::NotEqualCond { field_name, value } => {
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

#[derive(Clone)]
pub struct DataStore<T: KhronosContext> {
    executor: DataStoreExecutor<T>,
    ds_impl: Rc<dyn DataStoreImpl>,
    columns_cache: RefCell<Option<LuaValue>>
}

impl<T: KhronosContext> DataStore<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.executor.context.has_cap(&format!("datastore:{}", self.ds_impl.name())) && !self.executor.context.has_cap(&format!("datastore:{}:{}", self.ds_impl.name(), action)) {
            return Err(LuaError::runtime(format!(
                "Datastore action is not allowed in this template context: data store: {}, action: {}",
                self.ds_impl.name(),
                action
            )));
        }

        self.executor
            .datastore_provider
            .attempt_action(&action)
            .map_err(|e| LuaError::external(e.to_string()))?;

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for DataStore<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.ds_impl.name()));
        fields.add_field_method_get("table_name", |_, this| Ok(this.ds_impl.table_name()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("list", |_, this, ()| {
            Ok(
                lua_promise!(this, |lua, this|, {
                    this.ds_impl.list(lua).await
                })
            ) 
        });

        methods.add_method("columns", |lua, this, ()| {
            // Check for cached serialized data
            let mut cached_data = this
                .columns_cache
                .try_borrow_mut()
                .map_err(|e| LuaError::external(e.to_string()))?;

            if let Some(v) = cached_data.as_ref() {
                return Ok(v.clone());
            }

            let v = lua.to_value_with(&this.ds_impl.columns(), LUA_SERIALIZE_OPTIONS)?;

            *cached_data = Some(v.clone());

            Ok(v)
        });

        methods.add_method("get", |_, this, filters: LuaValue| {
            Ok(
                lua_promise!(this, filters, |lua, this, filters|, {
                    let filters: Filters = lua.from_value(filters)?;
                    this.ds_impl.get(lua, filters).await
                })
            ) 
        });

        methods.add_method("insert", |_, this, data: LuaValue| {
            Ok(
                lua_promise!(this, data, |lua, this, data|, {
                    this.ds_impl.insert(lua, data).await
                })
            ) 
        });

        methods.add_method("update", |_, this, (filters, data): (LuaValue, LuaValue)| {
            Ok(
                lua_promise!(this, filters, data, |lua, this, filters, data|, {
                    let filters: Filters = lua.from_value(filters)?;
                    this.ds_impl.update(lua, filters, data).await
                })
            ) 
        });

        methods.add_method("delete", |_, this, filters: LuaValue| {
            Ok(
                lua_promise!(this, filters, |lua, this, filters|, {
                    let filters: Filters = lua.from_value(filters)?;
                    this.ds_impl.delete(lua, filters).await
                })
            ) 
        });

        methods.add_method("count", |_, this, filters: LuaValue| {
            Ok(
                lua_promise!(this, filters, |lua, this, filters|, {
                    let filters: Filters = lua.from_value(filters)?;
                    this.ds_impl.count(lua, filters).await
                })
            ) 
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<DataStore<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "name",
                    "table_name",
                    // Methods
                    "columns",
                    "list",
                    "get",
                    "insert",
                    "update",
                    "delete",
                    "count",
                ],
            )
        });
    }
}

#[derive(Clone)]
pub struct DataStoreExecutor<T: KhronosContext> {
    context: T,
    datastore_provider: T::DataStoreProvider,
}

impl<T: KhronosContext> LuaUserData for DataStoreExecutor<T> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get", |_, this, name: String| {
            let Some(ds_impl) = this.datastore_provider.get_builtin_data_store(&name) else {
                return Ok(None);
            };
            let ds = DataStore {
                executor: this.clone(),
                ds_impl,
                columns_cache: RefCell::new(None)
            };
            Ok(Some(ds))
        });

        methods.add_method("public_list", |_, this, ()| {
            Ok(this.datastore_provider.public_builtin_data_stores())
        });
    }
}

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "new",
        lua.create_function(
            |_, (token, scope): (TemplateContextRef<T>, Option<String>)| {
                let scope = ExecutorScope::scope_str(scope)?;
                let Some(datastore_provider) = token.context.datastore_provider(scope) else {
                    return Err(LuaError::external(
                        "The datastore plugin is not supported in this context",
                    ));
                };

                let executor = DataStoreExecutor {
                    context: token.context.clone(),
                    datastore_provider,
                };

                Ok(executor)
            },
        )?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}

