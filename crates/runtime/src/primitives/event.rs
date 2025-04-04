use crate::plugins::antiraid::LUA_SERIALIZE_OPTIONS;
use mlua::prelude::*;
use std::{cell::RefCell, sync::Arc};

use super::create_userdata_iterator_with_fields;

#[derive(serde::Serialize, serde::Deserialize)]
struct InnerEvent {
    /// The name of the base event
    base_name: String,
    /// The name of the event
    name: String,
    /// The inner data of the object
    data: serde_json::Value,
    /// The author, if any, of the event
    author: Option<String>,
}

/// An `CreateEvent` is a/an thread-safe object that can be used to create a Event in multithreaded programs
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateEvent {
    inner: Arc<InnerEvent>,
}

impl CreateEvent {
    /// Create a new Event
    pub fn new(
        base_name: String,
        name: String,
        data: serde_json::Value,
        author: Option<String>,
    ) -> Self {
        Self {
            inner: Arc::new(InnerEvent {
                base_name,
                name,
                data,
                author,
            }),
        }
    }
}

impl CreateEvent {
    /// Returns the base name of the event
    pub fn base_name(&self) -> &str {
        &self.inner.base_name
    }

    /// Returns the name (NOT the base name) of the event
    pub fn name(&self) -> &str {
        &self.inner.name
    }
}

/// An `Event` is an object that can be passed to provide data to a Lua script.
pub struct Event {
    /// The inner data of the object
    inner: Arc<InnerEvent>,
    /// The cached serialized value of the event data
    cached_data: RefCell<Option<LuaValue>>,
}

impl Event {
    /// Converts the `CreateEvent` into an `Event`
    pub fn from_create_event(ce: &CreateEvent) -> Self {
        Self {
            inner: ce.inner.clone(),
            cached_data: RefCell::new(None),
        }
    }

    fn get_cached_data(&self, lua: &Lua) -> LuaResult<LuaValue> {
        // Check for cached serialized data
        let mut cached_data = self
            .cached_data
            .try_borrow_mut()
            .map_err(|e| LuaError::external(e.to_string()))?;

        if let Some(v) = cached_data.as_ref() {
            return Ok(v.clone());
        }

        let v = lua.to_value_with(&self.inner.data, LUA_SERIALIZE_OPTIONS)?;

        *cached_data = Some(v.clone());

        Ok(v)
    }
}

impl LuaUserData for Event {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("base_name", |lua, this| {
            let base_name = lua.to_value_with(&this.inner.base_name, LUA_SERIALIZE_OPTIONS)?;
            Ok(base_name)
        });
        fields.add_field_method_get("name", |lua, this| {
            let name = lua.to_value_with(&this.inner.name, LUA_SERIALIZE_OPTIONS)?;
            Ok(name)
        });
        fields.add_field_method_get("data", |lua, this| {
            let data = this.get_cached_data(lua)?;
            Ok(data)
        });
        fields.add_field_method_get("author", |lua, this| {
            let author = lua.to_value_with(&this.inner.author, LUA_SERIALIZE_OPTIONS)?;
            Ok(author)
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<Event>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "base_name",
                    "name",
                    "data",
                    "author",
                ],
            )
        });
    }
}
