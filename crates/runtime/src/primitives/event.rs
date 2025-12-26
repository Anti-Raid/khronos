use std::{cell::RefCell, rc::Rc};

use mluau::prelude::*;
use crate::plugins::antiraid::LUA_SERIALIZE_OPTIONS;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
enum InnerEventData {
    /// The inner data of the object
    Json(serde_json::Value),
    RawValue(Box<serde_json::value::RawValue>),
}

impl<'de> Deserialize<'de> for InnerEventData {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Ok(Self::Json(value))
    }
}

// Workaround for RawValue
impl Serialize for InnerEventData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            InnerEventData::Json(value) => value.serialize(serializer),
            InnerEventData::RawValue(raw_value) => {
                let value: serde_json::Value = serde_json::from_str(raw_value.get())
                    .map_err(serde::ser::Error::custom)?;
                value.serialize(serializer)
            },
        }
    }
}

/// An `CreateEvent` is a/an thread-safe object that can be used to create a Event
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateEvent {
    /// The name of the event
    name: String,
    /// The inner data of the object
    data: InnerEventData,
}

impl CreateEvent {
    /// Create a new Event
    pub fn new(
        name: String,
        data: serde_json::Value,
    ) -> Self {
        Self {
            name,
            data: InnerEventData::Json(data),
        }
    }

    /// Create a new Event given a raw value
    pub fn new_raw_value(
        name: String,
        data: Box<serde_json::value::RawValue>,
    ) -> Self {
        Self {
            name,
            data: InnerEventData::RawValue(data),
        }
    }

    fn into_lua(&self, lua: &Lua) -> LuaResult<LuaValue> {
        let tab = lua.create_table()?;
        tab.set("name", self.name.as_str())?;
        tab.set(
            "data",
            match self.data {
                InnerEventData::Json(ref value) => {
                    lua.to_value_with(value, LUA_SERIALIZE_OPTIONS)?
                },
                InnerEventData::RawValue(ref raw_value) => {
                    let value: serde_json::Value = serde_json::from_str(raw_value.get())
                        .map_err(|e| LuaError::external(e))?;
                    lua.to_value_with(&value, LUA_SERIALIZE_OPTIONS)?
                },
            },
        )?;
        tab.set_readonly(true);
        Ok(LuaValue::Table(tab))
    }
}

impl CreateEvent {
    /// Returns the event name
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn into_context(self) -> ContextEvent {
        ContextEvent::new(self)
    }
}

/// A reference to an event's data
#[derive(Clone)]
pub struct ContextEvent {
    pub(crate) event: Rc<RefCell<CreateEvent>>,
    pub(crate) cached_event_value: Rc<RefCell<Option<LuaValue>>>,
}

impl ContextEvent {
    pub fn new(event: CreateEvent) -> Self {
        Self {
            event: Rc::new(RefCell::new(event)),
            cached_event_value: Rc::default(),
        }
    }

    /// Turns the event into a LuaValue if not already converted, otherwise returns the cached value
    pub fn to_event_value(&self, lua: &Lua) -> LuaResult<LuaValue> {
        // Check for cached event value
        let mut cached_event_value = self
            .cached_event_value
            .try_borrow_mut()
            .map_err(|e| LuaError::external(e.to_string()))?;

        if let Some(v) = cached_event_value.as_ref() {
            return Ok(v.clone());
        }

        let event = self
            .event
            .try_borrow()
            .map_err(|e| LuaError::external(e.to_string()))?;
        
        let v = event.into_lua(lua)?;
        match v {
            LuaValue::Table(ref t) => {
                t.set_readonly(true);
            }
            _ => {}
        };

        *cached_event_value = Some(v.clone());

        Ok(v)
    }
}
