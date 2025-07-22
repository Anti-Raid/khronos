use crate::plugins::antiraid::LUA_SERIALIZE_OPTIONS;
use mluau::prelude::*;
use std::cell::RefCell;

use serde::{Deserialize, Serialize};

/// Represents data that is only serialized to Lua upon first access
///
/// This can be much more efficient than serializing the data every time it is accessed
pub struct Lazy<T: Serialize + 'static> {
    pub data: T,
    cached_data: RefCell<Option<LuaValue>>,
}

impl<T: serde::Serialize + 'static> Lazy<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            cached_data: RefCell::new(None),
        }
    }
}

// A T can be converted to a Lazy<T> by just wrapping it
impl<T: serde::Serialize> From<T> for Lazy<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

// Ensure Lazy<T> serializes to T
impl<T: serde::Serialize> Serialize for Lazy<T> {
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

impl<T: serde::Serialize + Clone> Clone for Lazy<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            cached_data: self.cached_data.clone(),
        }
    }
}

impl<T: serde::Serialize + std::fmt::Debug> std::fmt::Debug for Lazy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lazy").field("data", &self.data).finish()
    }
}

// A Lazy<T> is a LuaUserData
impl<T: serde::Serialize + 'static> LuaUserData for Lazy<T> {
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

        fields.add_meta_field(LuaMetaMethod::Type, "Lazy");
    }

    #[cfg(feature = "repl")]
    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
    }
}
