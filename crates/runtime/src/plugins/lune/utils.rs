#![allow(clippy::missing_errors_doc)]

/// From https://github.com/lune-org/lune/blob/main/crates/lune-utils/src/table_builder.rs
///
/// SPDX-License-Identifier: MPL-2.0
use std::future::Future;

use mluau::{prelude::*, MaybeSend};
use mlua_scheduler::LuaSchedulerAsync;

/**
    Utility struct for building Lua tables.
*/
pub struct TableBuilder<'lua> {
    lua: &'lua Lua,
    tab: LuaTable,
}

#[allow(dead_code)]
impl<'lua> TableBuilder<'lua> {
    /**
        Creates a new table builder.
    */
    pub fn new(lua: &'lua Lua) -> LuaResult<Self> {
        let tab = lua.create_table()?;
        Ok(Self { lua, tab })
    }

    /**
        Adds a new key-value pair to the table.

        This will overwrite any value that already exists.
    */
    pub fn with_value<K, V>(self, key: K, value: V) -> LuaResult<Self>
    where
        K: IntoLua,
        V: IntoLua,
    {
        self.tab.raw_set(key, value)?;
        Ok(self)
    }

    /**
        Adds multiple key-value pairs to the table.

        This will overwrite any values that already exist.
    */
    pub fn with_values<K, V>(self, values: Vec<(K, V)>) -> LuaResult<Self>
    where
        K: IntoLua,
        V: IntoLua,
    {
        for (key, value) in values {
            self.tab.raw_set(key, value)?;
        }
        Ok(self)
    }

    /**
        Adds a new key-value pair to the sequential (array) section of the table.

        This will not overwrite any value that already exists,
        instead adding the value to the end of the array.
    */
    pub fn with_sequential_value<V>(self, value: V) -> LuaResult<Self>
    where
        V: IntoLua,
    {
        self.tab.raw_push(value)?;
        Ok(self)
    }

    /**
        Adds multiple values to the sequential (array) section of the table.

        This will not overwrite any values that already exist,
        instead adding the values to the end of the array.
    */
    pub fn with_sequential_values<V>(self, values: Vec<V>) -> LuaResult<Self>
    where
        V: IntoLua,
    {
        for value in values {
            self.tab.raw_push(value)?;
        }
        Ok(self)
    }

    /**
        Adds a new key-value pair to the table, with a function value.

        This will overwrite any value that already exists.
    */
    pub fn with_function<K, A, R, F>(self, key: K, func: F) -> LuaResult<Self>
    where
        K: IntoLua,
        A: FromLuaMulti,
        R: IntoLuaMulti,
        F: Fn(&Lua, A) -> LuaResult<R> + Send + Sync + 'static,
    {
        let f = self.lua.create_function(func)?;
        self.with_value(key, LuaValue::Function(f))
    }

    /**
        Adds a new key-value pair to the table, with an async function value.

        This will overwrite any value that already exists.
    */
    pub fn with_async_function<K, A, R, F, FR>(self, key: K, func: F) -> LuaResult<Self>
    where
        K: IntoLua,
        A: FromLuaMulti + MaybeSend + 'static,
        R: IntoLuaMulti + 'static,
        F: Fn(Lua, A) -> FR + MaybeSend + Clone + 'static,
        FR: Future<Output = LuaResult<R>> + MaybeSend + 'static,
    {
        let f = self.lua.create_scheduler_async_function(func)?;
        self.with_value(key, LuaValue::Function(f))
    }

    /**
        Adds a metatable to the table.

        This will overwrite any metatable that already exists.
    */
    pub fn with_metatable(self, table: LuaTable) -> LuaResult<Self> {
        self.tab.set_metatable(Some(table))?;
        Ok(self)
    }

    /**
        Builds the table as a read-only table.

        This will prevent any *direct* modifications to the table.
    */
    pub fn build_readonly(self) -> LuaResult<LuaTable> {
        self.tab.set_readonly(true);
        Ok(self.tab)
    }

    /**
        Builds the table.
    */
    pub fn build(self) -> LuaResult<LuaTable> {
        Ok(self.tab)
    }
}
