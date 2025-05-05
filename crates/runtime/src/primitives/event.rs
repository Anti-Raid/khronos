use crate::plugins::antiraid::LUA_SERIALIZE_OPTIONS;
use mlua::prelude::*;
use std::sync::Arc;

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
#[derive(Clone)]
pub struct Event {
    /// The inner data of the object
    inner: Arc<InnerEvent>,
}

impl Event {
    /// Converts the `CreateEvent` into an `Event`
    pub fn from_create_event(ce: &CreateEvent) -> Self {
        Self {
            inner: ce.inner.clone(),
        }
    }
}

impl IntoLua for Event {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let tab = lua.create_table()?;
        tab.set("base_name", self.inner.base_name.clone())?;
        tab.set("name", self.inner.name.clone())?;
        tab.set(
            "data",
            lua.to_value_with(&self.inner.data, LUA_SERIALIZE_OPTIONS)?,
        )?;
        tab.set("author", self.inner.author.clone())?;
        tab.set_readonly(true);
        Ok(LuaValue::Table(tab))
    }
}
