use crate::plugins::antiraid::LUA_SERIALIZE_OPTIONS;
use mlua::prelude::*;
use std::cell::RefCell;

use serde::{Deserialize, Serialize};

use crate::primitives::create_userdata_iterator_with_fields;

/// Represents data that is only serialized to Lua upon first access
///
/// This can be much more efficient than serializing the data every time it is accessed
pub struct Lazy<T: Serialize + for<'de> Deserialize<'de> + 'static> {
    pub data: T,
    cached_data: RefCell<Option<LuaValue>>,
}

impl<T: serde::Serialize + for<'de> Deserialize<'de> + 'static> Lazy<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            cached_data: RefCell::new(None),
        }
    }
}

// A T can be converted to a Lazy<T> by just wrapping it
impl<T: serde::Serialize + for<'de> Deserialize<'de>> From<T> for Lazy<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

// Ensure Lazy<T> serializes to T
impl<T: serde::Serialize + for<'de> Deserialize<'de>> Serialize for Lazy<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.data.serialize(serializer)
    }
}

// Ensure Lazy<T> deserializes from T
impl<'de, T: serde::Serialize + for<'a> Deserialize<'a>> Deserialize<'de> for Lazy<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::new(T::deserialize(deserializer)?))
    }
}

impl<'de, T: serde::Serialize + for<'a> Deserialize<'a> + Clone> Clone for Lazy<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            cached_data: self.cached_data.clone(),
        }
    }
}

impl<'de, T: serde::Serialize + for<'a> Deserialize<'a> + std::fmt::Debug> std::fmt::Debug for Lazy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lazy")
            .field("data", &self.data)
            .finish()
    }
}

// A Lazy<T> is a LuaUserData
impl<T: serde::Serialize + for<'de> Deserialize<'de> + 'static> LuaUserData for Lazy<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // Returns the data, serializing it if it hasn't been serialized yet
        fields.add_field_method_get("data", |lua, this| {
            // Check for cached serialized data
            let mut cached_data = this
                .cached_data
                .try_borrow_mut()
                .map_err(|e| LuaError::external(e.to_string()))?;

            if let Some(v) = cached_data.as_ref() {
                return Ok(v.clone());
            }

            let v = lua.to_value_with(&this.data, LUA_SERIALIZE_OPTIONS)?;

            *cached_data = Some(v.clone());

            Ok(v)
        });

        // Always returns true. Allows the user to check if the data is a lazy or not
        fields.add_field_method_get("lazy", |_lua, _this| Ok(true));

        fields.add_meta_field(LuaMetaMethod::Type, "Lazy");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<Lazy<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "lazy", "data",
                ],
            )
        });
    }
}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    // For the cases where you want to just make your own lazy data. Might be more useful
    // in the future as well.
    module.set(
        "new",
        lua.create_function(|lua, (data,): (LuaValue,)| {
            let val: serde_json::Value = lua.from_value(data).map_err(LuaError::external)?;

            Ok(Lazy::new(val))
        })?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
