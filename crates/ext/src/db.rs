pub trait DbRow {
    fn row(&self) -> &sqlx::postgres::PgRow;
}

/// Helper method to map a DbRow into a type T that derives sqlx::FromRow
pub fn map_db_row<T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow>>(row: &impl DbRow) -> sqlx::Result<T> {
    T::from_row(row.row())
}

#[macro_export]
/// Macro to create a database plugin with a list of supported types and their conversions.
macro_rules! db_plugin {
    ($($type:ty => { $base:ident, $opt:ident, $list:ident, $typestr:literal, |$lua:ident, $val:ident| $luaconv:block, |$luaf:ident, $opaque:ident| $luaconvf:block }),* $(,)?) => {
        use ::serde::{Serialize, Deserialize};
        use $crate::sqlx_ext::{Row, PgPool};
        use $crate::mluau_ext::prelude::*;
        use $crate::mlua_scheduler_ext::LuaSchedulerAsyncUserData;
        use $crate::db::DbRow;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(tag = "type", content = "value")]
        #[allow(dead_code)]
        pub enum DbValue {
            $( 
                $base($type),
                $opt(Option<$type>),
                $list(Vec<$type>),
            )*
        }

        #[allow(dead_code)]
        struct DbValueTaker(Vec<DbValue>);
        impl FromLua for DbValueTaker {
            fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
                let mut values = Vec::new();
                match value {
                    LuaValue::UserData(ud) => {
                        let Ok(ov) = ud.borrow::<DbValue>() else {
                            return Err(LuaError::external("Expected DbValue userdata"));
                        };
                        values.push(ov.clone());
                    }
                    _ => return Err(LuaError::external("Expected a table of DbValue userdata")),
                }
                Ok(DbValueTaker(values))
            }
        }

        #[allow(dead_code)]
        impl DbValue {
            /// Creates an DbValue from a database row, given the column index and expected type name.
            pub fn from_row(row: &sqlx::postgres::PgRow, idx: usize, type_name: &str) -> sqlx::Result<Self> {
                match type_name {
                    $(
                        $typestr => {
                            let val = row.try_get::<$type, _>(idx)?;
                            Ok(DbValue::$base(val))
                        },
                        concat!($typestr, "?") => {
                            let val = row.try_get::<Option<$type>, _>(idx)?;
                            Ok(DbValue::$opt(val))
                        },
                        concat!("{", $typestr, "}") => {
                            let val = row.try_get::<Vec<$type>, _>(idx)?;
                            Ok(DbValue::$list(val))
                        },
                    )*
                    _ => Err(sqlx::Error::ColumnNotFound(format!("Unknown type for DbValue conversion: {}", type_name))),
                }
            }
            
            /// Creates an DbValue from a Lua value, given the expected type name.
            pub fn from_lua(lua: &Lua, value: LuaValue, type_name: &str) -> LuaResult<Self> {
                match type_name {
                    $(
                        $typestr => {
                            let func = |$lua: &Lua, $val: LuaValue| $luaconv;
                            let val: $type = func(lua, value)?;
                            Ok(DbValue::$base(val))
                        },
                        concat!($typestr, "?") => {
                            let func = |$lua: &Lua, $val: LuaValue| $luaconv;
                            if let LuaValue::Nil = value {
                                return Ok(DbValue::$opt(None));
                            }
                            let val: $type = func(lua, value)?;
                            Ok(DbValue::$opt(Some(val)))
                        },
                        concat!("{", $typestr, "}") => {
                            let func = |$lua: &Lua, $val: LuaValue| $luaconv;
                            match value {
                                LuaValue::Table(table) => {
                                    let mut vec = Vec::new();
                                    table.for_each_value::<LuaValue>(|v| {
                                        let val: $type = func(lua, v)?;
                                        vec.push(val);
                                        Ok(())
                                    })?;
                                    Ok(DbValue::$list(vec))
                                }
                                _ => return Err(LuaError::external(format!("Expected a table for type {{}}: {}", $typestr))),
                            }
                        },
                    )*
                    _ => Err(LuaError::external(format!("Unknown type for DbValue conversion: {}", type_name))),
                }
            }

            /// Converts the DbValue back into a Lua value.
            pub fn into_lua(&self, lua: &Lua) -> LuaResult<LuaValue> {
                match self {
                    $(
                        DbValue::$base(v) => {
                            let func = |$luaf: &Lua, $opaque: &$type| $luaconvf;
                            func(lua, v)
                        },
                        DbValue::$opt(v) => {
                            let func = |$luaf: &Lua, $opaque: &$type| $luaconvf;
                            let Some(v) = v else {
                                return Ok(LuaValue::Nil);
                            };
                            func(lua, v)
                        },
                        DbValue::$list(v) => {
                            let func = |$luaf: &Lua, $opaque: &$type| $luaconvf;
                            let table = lua.create_table()?;
                            for item in v.iter() {
                                let lua_val = func(lua, item)?;
                                table.push(lua_val)?;
                            }
                            table.set_readonly(true);
                            Ok(LuaValue::Table(table))
                        },
                    )*
                }
            }

            pub fn type_name(&self) -> &'static str {
                match self {
                    $(
                        DbValue::$base(_) => $typestr,
                        DbValue::$opt(_) => concat!($typestr, "?"),
                        DbValue::$list(_) => concat!("{", $typestr, "}"),
                    )*
                }
            }

            pub fn bind(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> 
                sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
            {
                match self {
                    $(
                        DbValue::$base(v) => query.bind(v),
                        DbValue::$opt(v) => query.bind(v),
                        DbValue::$list(v) => query.bind(v),
                    )*
                }
            }
        }

        impl LuaUserData for DbValue {
            fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
                methods.add_method("type", |_, this, ()| {
                    Ok(this.type_name())
                });
                methods.add_method("get", |lua, this, ()| {
                    this.into_lua(lua)
                });
            }
        }

        #[allow(dead_code)]
        pub struct Db {
            pool: PgPool,
        }

        #[allow(dead_code)]
        impl Db {
            pub fn new(pool: PgPool) -> Self {
                Self { pool }
            }

            pub fn pool(&self) -> &PgPool {
                &self.pool
            }
        }

        impl LuaUserData for Db {
            fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
                methods.add_method("cast", |lua, _this: &Db, (value, typ): (LuaValue, String)| {
                    DbValue::from_lua(lua, value, &typ)
                });

                methods.add_scheduler_async_method("fetchall", async |_lua, this, (query, params): (String, DbValueTaker)| {
                    let mut q = sqlx::query(&query);
                    for param in params.0 {
                        q = param.bind(q);
                    }
                    let rows = q.fetch_all(&this.pool).await.map_err(|e| LuaError::external(format!("Database query failed: {}", e)))?;
                    Ok(rows.into_iter().map(PgRow::from_row).collect::<Vec<_>>())
                });
            }
        }

        #[allow(dead_code)]
        pub struct PgRow {
            row: sqlx::postgres::PgRow,
        }

        #[allow(dead_code)]
        impl PgRow {
            pub fn from_row(row: sqlx::postgres::PgRow) -> Self {
                Self { row }
            }

            pub fn row(&self) -> &sqlx::postgres::PgRow {
                &self.row
            }
        }

        impl DbRow for PgRow {
            fn row(&self) -> &sqlx::postgres::PgRow {
                &self.row
            }
        }

        impl LuaUserData for PgRow {
            fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
                methods.add_method("get", |_lua, this, (idx, typ): (usize, String)| {
                    DbValue::from_row(&this.row, idx as usize, &typ).map_err(|e| LuaError::external(format!("Failed to get column {}: {}", idx, e)))
                });
            }
        }
    };
}

mod test {
    db_plugin! {
        i32 => { I32, I32Opt, I32List, "i32", |lua, value| { lua.from_value(value) }, |lua, opaque| { lua.to_value(opaque) } },
        i64 => { I64, I64Opt, I64List, "i64", |lua, value| { lua.from_value(value) }, |lua, opaque| { lua.to_value(&opaque) } },
        String => { String, StringOpt, StringList, "string", |lua, value| { lua.from_value(value) }, |lua, opaque| { lua.to_value(opaque) } },
        bool => { Bool, BoolOpt, BoolList, "boolean", |lua, value| { lua.from_value(value) }, |_lua, opaque| { Ok::<_, LuaError>(LuaValue::Boolean(*opaque)) } },
        f64 => { F64, F64Opt, F64List, "f64", |lua, value| { lua.from_value(value) }, |lua, opaque| { lua.to_value(opaque) } },
    }
}