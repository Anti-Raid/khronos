use mluau::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::{PgPool, Postgres, postgres::PgArguments};
use sqlx::query::Query;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use khronos_runtime::core::datetime::DateTimeUtc as LuaDateTime;

pub trait DbRow {
    fn row(&self) -> &sqlx::postgres::PgRow;
}

/// Helper method to map a DbRow into a type T that derives sqlx::FromRow
pub fn map_db_row<T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow>>(row: &impl DbRow) -> sqlx::Result<T> {
    T::from_row(row.row())
}

/// A `DbValueMapper` maps values between Lua and the underlying postgres types
pub trait DbValueMapper: 'static + Clone + Sized {
    fn supports(type_name: &str) -> bool;
    fn type_name(&self) -> &'static str;

    // Map to/from postgres
    fn bind(self, query: Query<'_, Postgres, PgArguments>) -> Query<'_, Postgres, PgArguments>;
    fn from_row(row: &sqlx::postgres::PgRow, idx: usize, type_name: &str) -> sqlx::Result<Self>;

    // Map to/from Lua
    fn from_lua(lua: &Lua, value: LuaValue, type_name: &str) -> LuaResult<Self>;
    fn to_lua(&self, lua: &Lua) -> LuaResult<LuaValue>;
}

#[derive(Clone)]
struct DbValue<T: DbValueMapper>(T);

impl<T: DbValueMapper> DbValue<T> {
    /// Creates an DbValue from a database row, given the column index and expected type name.
    fn from_row(row: &sqlx::postgres::PgRow, idx: usize, type_name: &str) -> sqlx::Result<Self> {
        T::from_row(row, idx, type_name).map(DbValue)
    }

    /// Creates an DbValue from a Lua value, given the expected type name.
    fn from_lua(lua: &Lua, value: LuaValue, type_name: &str) -> LuaResult<Self> {
        T::from_lua(lua, value, type_name).map(DbValue)
    }

    /// Binds the value to a sqlx query.
    fn bind(self, query: Query<'_, Postgres, PgArguments>) -> Query<'_, Postgres, PgArguments>
    {
        self.0.bind(query)
    }
}

impl<T: DbValueMapper> LuaUserData for DbValue<T> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("type", |_, this, ()| {
            Ok(this.0.type_name())
        });
        methods.add_method("get", |lua, this, ()| {
            this.0.to_lua(lua)
        });
    }
}

/// A wrapper around a sqlx Row that implements DbRow and provides a method to get columns as DbValue<T> using the provided DbValueMapper
pub struct PgRow<T: DbValueMapper> {
    row: sqlx::postgres::PgRow,
    _marker: std::marker::PhantomData<T>,
}

#[allow(dead_code)]
impl<T: DbValueMapper> PgRow<T> {
    pub fn from_row(row: sqlx::postgres::PgRow) -> Self {
        Self { row, _marker: std::marker::PhantomData }
    }

    pub fn row(&self) -> &sqlx::postgres::PgRow {
        &self.row
    }
}

impl<T: DbValueMapper> DbRow for PgRow<T> {
    fn row(&self) -> &sqlx::postgres::PgRow {
        &self.row
    }
}

impl<T: DbValueMapper> LuaUserData for PgRow<T> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get", |_lua, this, (idx, typ): (usize, String)| {
            DbValue::<T>::from_row(&this.row, idx as usize, &typ).map_err(|e| LuaError::external(format!("Failed to get column {}: {}", idx, e)))
        });
    }
}

/// A helper struct to take a list of DbValue from Lua, since we can't directly implement FromLua for Vec<DbValue<T>> 
/// (and may want some special behavior in the future, like allowing a single value to be passed where a list is expected and automatically converting it)
struct DbValueTaker<T: DbValueMapper>(DbValue<T>);
impl<T: DbValueMapper> FromLua for DbValueTaker<T> {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::UserData(ud) => {
                let Ok(ov) = ud.borrow::<DbValue<T>>() else {
                    return Err(LuaError::external("Expected DbValue userdata"));
                };
                Ok(DbValueTaker(ov.clone()))
            },
            _ => return Err(LuaError::external("Expected a DbValue userdata")),
        }
    }
}

/// A wrapper around a sqlx PgPool that provides methods to execute queries and fetch results, with values mapped through the DbValueMapper
pub struct Db<T: DbValueMapper> {
    pool: PgPool,
    _marker: std::marker::PhantomData<T>,
}

#[allow(dead_code)]
impl<T: DbValueMapper> Db<T> {
    pub fn new(pool: PgPool) -> Self {
        Self { pool, _marker: std::marker::PhantomData }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl<T: DbValueMapper> LuaUserData for Db<T> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("supports", |_lua, _this, typ: String| {
            Ok(T::supports(&typ))
        });

        methods.add_method("cast", |lua, _this, (value, typ): (LuaValue, String)| {
            DbValue::<T>::from_lua(lua, value, &typ)
        });

        methods.add_scheduler_async_method("execute", async |_lua, this, (query, params): (String, Vec<DbValueTaker<T>>)| {
            let mut q = sqlx::query(&query);
            for param in params {
                q = param.0.bind(q);
            }
            
            let result = q.execute(&this.pool).await.map_err(|e| LuaError::external(format!("Database execute failed: {}", e)))?;
                
            Ok(result.rows_affected())
        });

        methods.add_scheduler_async_method("fetchall", async |_lua, this, (query, params): (String, Vec<DbValueTaker<T>>)| {
            let mut q = sqlx::query(&query);
            for param in params {
                q = param.0.bind(q);
            }
            let rows = q.fetch_all(&this.pool).await.map_err(|e| LuaError::external(format!("Database query failed: {}", e)))?;
            Ok(rows.into_iter().map(PgRow::<T>::from_row).collect::<Vec<_>>())
        });

        // Spawns a transaction and returns the wrapper
        methods.add_scheduler_async_method("begin", async |_lua, this, ()| {
            let tx = this.pool.begin().await.map_err(|e| LuaError::external(format!("Failed to begin transaction: {}", e)))?;
            Ok(DbTx::<T>::new(tx))
        });
    }
}

#[derive(Clone)]
/// A wrapper around a sqlx Transaction that provides methods to execute queries and fetch results, with values mapped through the DbValueMapper
/// 
/// The transaction is wrapped in an Arc<Mutex<Option<>>> to allow taking ownership of it when committing or rolling back, while still allowing the DbTx struct to be cloned and used across async calls. Once the transaction is committed or rolled back, the Option is set to None to prevent further use.
pub struct DbTx<T: DbValueMapper> {
    tx: std::sync::Arc<tokio::sync::Mutex<Option<sqlx::Transaction<'static, sqlx::Postgres>>>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DbValueMapper> DbTx<T> {
    pub fn new(tx: sqlx::Transaction<'static, sqlx::Postgres>) -> Self {
        Self { tx: std::sync::Arc::new(tokio::sync::Mutex::new(Some(tx))), _marker: std::marker::PhantomData }
    }
}

impl<T: DbValueMapper> LuaUserData for DbTx<T> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_scheduler_async_method("execute", async |_lua, this, (query, params): (String, Vec<DbValueTaker<T>>)| {
            let mut guard = this.tx.lock().await;
            let tx = guard.as_mut().ok_or_else(|| LuaError::external("Transaction already committed or rolled back"))?;
            
            let mut q = sqlx::query(&query);
            for param in params {
                q = param.0.bind(q);
            }
            
            let result = q.execute(&mut **tx).await.map_err(|e| LuaError::external(format!("Transaction execute failed: {}", e)))?;
                
            Ok(result.rows_affected())
        });

        methods.add_scheduler_async_method("fetchall", async |_lua, this, (query, params): (String, Vec<DbValueTaker<T>>)| {
            let mut guard = this.tx.lock().await;
            let tx = guard.as_mut().ok_or_else(|| LuaError::external("Transaction already committed or rolled back"))?;
            
            let mut q = sqlx::query(&query);
            for param in params {
                q = param.0.bind(q);
            }
            
            let rows = q.fetch_all(&mut **tx).await.map_err(|e| LuaError::external(format!("Transaction query failed: {}", e)))?;
            Ok(rows.into_iter().map(PgRow::<T>::from_row).collect::<Vec<_>>())
        });

        methods.add_scheduler_async_method("commit", async |_lua, this, ()| {
            // Take ownership of the transaction out of the Option
            let tx_opt = this.tx.lock().await.take();
            if let Some(tx) = tx_opt {
                tx.commit().await.map_err(|e| LuaError::external(format!("Failed to commit transaction: {}", e)))?;
            } else {
                return Err(LuaError::external("Transaction already completed"));
            }
            Ok(())
        });

        methods.add_scheduler_async_method("rollback", async |_lua, this, ()| {
            // Take ownership of the transaction out of the Option
            let tx_opt = this.tx.lock().await.take();
            if let Some(tx) = tx_opt {
                tx.rollback().await.map_err(|e| LuaError::external(format!("Failed to rollback transaction: {}", e)))?;
            } else {
                return Err(LuaError::external("Transaction already completed"));
            }
            Ok(())
        });
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
/// A simple db value mapper that supports common types like i32, i64, String, bool, f64, timestamptz, json, and jsonb, as well as lists of those types. 
/// 
/// This can be used for simple cases where you don't need any special behavior in the mapping and just want a straightforward way to convert between Lua values and database values.
pub enum SimpleDbValueMapper {
    I32(Option<i32>),
    I32List(Option<Vec<i32>>),
    I64(Option<i64>),
    I64List(Option<Vec<i64>>),
    String(Option<String>),
    StringList(Option<Vec<String>>),
    Bool(Option<bool>),
    BoolList(Option<Vec<bool>>),
    F64(Option<f64>),
    F64List(Option<Vec<f64>>),
    Timestamptz(Option<chrono::DateTime<chrono::Utc>>),
    TimestamptzList(Option<Vec<chrono::DateTime<chrono::Utc>>>),
    Json(Option<serde_json::Value>),
    JsonList(Option<Vec<serde_json::Value>>),
    Jsonb(Option<serde_json::Value>),
    JsonbList(Option<Vec<serde_json::Value>>),
}

impl DbValueMapper for SimpleDbValueMapper {
    fn supports(type_name: &str) -> bool {
        matches!(type_name, 
            "i32" | "{i32}" |
            "i64" | "{i64}" |
            "string" | "{string}" |
            "bool" | "{bool}" |
            "f64" | "{f64}" |
            "timestamptz" | "{timestamptz}" |
            "json" | "{json}" |
            "jsonb" | "{jsonb}"
        )
    }

    fn type_name(&self) -> &'static str {
        match self {
            Self::I32(_) => "i32",
            Self::I32List(_) => "{i32}",
            Self::I64(_) => "i64",
            Self::I64List(_) => "{i64}",
            Self::String(_) => "string",
            Self::StringList(_) => "{string}",
            Self::Bool(_) => "bool",
            Self::BoolList(_) => "{bool}",
            Self::F64(_) => "f64",
            Self::F64List(_) => "{f64}",
            Self::Timestamptz(_) => "timestamptz",
            Self::TimestamptzList(_) => "{timestamptz}",
            Self::Json(_) => "json",
            Self::JsonList(_) => "{json}",
            Self::Jsonb(_) => "jsonb",
            Self::JsonbList(_) => "{jsonb}",
        }
    }

    fn bind(self, query: Query<'_, Postgres, PgArguments>) -> Query<'_, Postgres, PgArguments> {
        match self {
            Self::I32(v) => query.bind(v),
            Self::I32List(v) => query.bind(v),
            Self::I64(v) => query.bind(v),
            Self::I64List(v) => query.bind(v),
            Self::String(v) => query.bind(v),
            Self::StringList(v) => query.bind(v),
            Self::Bool(v) => query.bind(v),
            Self::BoolList(v) => query.bind(v),
            Self::F64(v) => query.bind(v),
            Self::F64List(v) => query.bind(v),
            Self::Timestamptz(v) => query.bind(v),
            Self::TimestamptzList(v) => query.bind(v),
            Self::Json(v) => query.bind(v),
            Self::JsonList(v) => query.bind(v), 
            Self::Jsonb(v) => query.bind(v),
            Self::JsonbList(v) => query.bind(v),
        }
    }

    fn from_row(row: &sqlx::postgres::PgRow, idx: usize, type_name: &str) -> sqlx::Result<Self> {
        match type_name {
            "i32" => Ok(Self::I32(row.try_get(idx)?)),
            "{i32}" => Ok(Self::I32List(row.try_get(idx)?)),
            "i64" => Ok(Self::I64(row.try_get(idx)?)),
            "{i64}" => Ok(Self::I64List(row.try_get(idx)?)),
            "string" => Ok(Self::String(row.try_get(idx)?)),
            "{string}" => Ok(Self::StringList(row.try_get(idx)?)),
            "bool" => Ok(Self::Bool(row.try_get(idx)?)),
            "{bool}" => Ok(Self::BoolList(row.try_get(idx)?)),
            "f64" => Ok(Self::F64(row.try_get(idx)?)),
            "{f64}" => Ok(Self::F64List(row.try_get(idx)?)),
            "timestamptz" => Ok(Self::Timestamptz(row.try_get(idx)?)),
            "{timestamptz}" => Ok(Self::TimestamptzList(row.try_get(idx)?)),
            "json" => Ok(Self::Json(row.try_get(idx)?)),
            "{json}" => Ok(Self::JsonList(row.try_get(idx)?)),
            "jsonb" => Ok(Self::Jsonb(row.try_get(idx)?)),
            "{jsonb}" => Ok(Self::JsonbList(row.try_get(idx)?)),
            _ => Err(sqlx::Error::ColumnNotFound(format!("Unsupported type name: {}", type_name))),
        }
    }

    fn from_lua(lua: &Lua, value: LuaValue, type_name: &str) -> LuaResult<Self> {
        match type_name {
            "i32" => lua.from_value(value).map(Self::I32),
            "{i32}" => lua.from_value(value).map(Self::I32List),
            "i64" => lua.from_value(value).map(Self::I64),
            "{i64}" => lua.from_value(value).map(Self::I64List),
            "string" => lua.from_value(value).map(Self::String),
            "{string}" => lua.from_value(value).map(Self::StringList),
            "bool" => lua.from_value(value).map(Self::Bool),
            "{bool}" => lua.from_value(value).map(Self::BoolList),
            "f64" => lua.from_value(value).map(Self::F64),
            "{f64}" => lua.from_value(value).map(Self::F64List),
            "timestamptz" => {
                if let LuaValue::UserData(ref ud) = value {
                    if let Ok(ldt) = ud.borrow::<LuaDateTime>() {
                        return Ok(Self::Timestamptz(Some(ldt.to_utc())));
                    }
                }
                lua.from_value(value).map(Self::Timestamptz)
            },
            "{timestamptz}" => {                
                if let LuaValue::Table(ref table) = value {
                    let mut lst = Vec::with_capacity(table.raw_len());
                    for v in table.sequence_values::<LuaValue>() {
                        let v = v?;
                        if let LuaValue::UserData(ref ud) = v {
                            if let Ok(ldt) = ud.borrow::<LuaDateTime>() {
                                lst.push(ldt.to_utc());
                                continue;
                            }
                        } 

                        lst.push(lua.from_value(v)?);
                    }

                    return Ok(Self::TimestamptzList(Some(lst)));
                } 
                lua.from_value(value).map(Self::TimestamptzList)
            },
            "json" => lua.from_value(value).map(Self::Json),
            "{json}" => lua.from_value(value).map(Self::JsonList),
            "jsonb" => lua.from_value(value).map(Self::Jsonb),
            "{jsonb}" => lua.from_value(value).map(Self::JsonbList),
            _ => Err(LuaError::external(format!("Unsupported type name: {}", type_name))),
        }
    }

    fn to_lua(&self, lua: &Lua) -> LuaResult<LuaValue> {
        match self {
            Self::I32(v) => lua.to_value(v),
            Self::I32List(v) => lua.to_value(v),
            Self::I64(v) => lua.to_value(v),
            Self::I64List(v) => lua.to_value(v),
            Self::String(v) => lua.to_value(v),
            Self::StringList(v) => lua.to_value(v),
            Self::Bool(v) => lua.to_value(v),
            Self::BoolList(v) => lua.to_value(v),
            Self::F64(v) => lua.to_value(v),
            Self::F64List(v) => lua.to_value(v),
            Self::Timestamptz(v) => lua.to_value(v),
            Self::TimestamptzList(v) => lua.to_value(v),
            Self::Json(v) => lua.to_value(v),
            Self::JsonList(v) => lua.to_value(v),
            Self::Jsonb(v) => lua.to_value(v),
            Self::JsonbList(v) => lua.to_value(v),
        }
    }
}

/// Chain two DbValueMappers together, trying first left and then right if left fails
/// 
/// This allows you to have a flexible mapping strategy where you can support multiple different types and have some fallback 
/// behavior if a value doesn't match the expected type.
#[derive(Clone)]
pub enum ChainedDbValueMapper<T: DbValueMapper, U: DbValueMapper> {
    Left(T),
    Right(U),
}

impl<T: DbValueMapper, U: DbValueMapper> DbValueMapper for ChainedDbValueMapper<T, U> {
    fn supports(type_name: &str) -> bool {
        T::supports(type_name) || U::supports(type_name)
    }

    fn type_name(&self) -> &'static str {
        match self {
            Self::Left(left) => left.type_name(),
            Self::Right(right) => right.type_name(),
        }
    }

    fn bind(self, query: Query<'_, Postgres, PgArguments>) -> Query<'_, Postgres, PgArguments> {
        match self {
            Self::Left(left) => left.bind(query),
            Self::Right(right) => right.bind(query),
        }
    }

    fn from_row(row: &sqlx::postgres::PgRow, idx: usize, type_name: &str) -> sqlx::Result<Self> {
        if T::supports(type_name) {
            return T::from_row(row, idx, type_name).map(Self::Left);
        } else if U::supports(type_name) {
            return U::from_row(row, idx, type_name).map(Self::Right);
        } else {
            return Err(sqlx::Error::InvalidArgument(format!("Unsupported type name: {}", type_name)));
        }
    }

    fn from_lua(lua: &Lua, value: LuaValue, type_name: &str) -> LuaResult<Self> {
        if T::supports(type_name) {
            return T::from_lua(lua, value, type_name).map(Self::Left);
        } else if U::supports(type_name) {
            return U::from_lua(lua, value, type_name).map(Self::Right);
        } else {
            return Err(LuaError::external(format!("Unsupported type name: {}", type_name)));
        }
    }

    fn to_lua(&self, lua: &Lua) -> LuaResult<LuaValue> {
        match self {
            Self::Left(left) => left.to_lua(lua),
            Self::Right(right) => right.to_lua(lua),
        }
    }
}