use mluau::prelude::*;
use crate::rt::KhronosRuntime;
use crate::utils::khronos_value::KhronosValue;
use crate::plugins::antiraid::LUA_SERIALIZE_OPTIONS;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
enum InnerEventData {
    /// The inner data of the object
    Json(serde_json::Value),
    /// The inner data of the object, as a KhronosValue
    Khronos(KhronosValue),
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
            InnerEventData::Khronos(khronos_value) => khronos_value.serialize(serializer),
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
    /// The name of the base event
    base_name: String,
    /// The name of the event
    name: String,
    /// The inner data of the object
    data: InnerEventData,
}

impl CreateEvent {
    /// Create a new Event
    pub fn new(
        base_name: String,
        name: String,
        data: serde_json::Value,
    ) -> Self {
        Self {
            base_name,
            name,
            data: InnerEventData::Json(data),
        }
    }

    /// Create a new Event given a raw value
    pub fn new_raw_value(
        base_name: String,
        name: String,
        data: Box<serde_json::value::RawValue>,
    ) -> Self {
        Self {
            base_name,
            name,
            data: InnerEventData::RawValue(data),
        }
    }

    /// Create a new Event given a KhronosValue
    pub fn new_khronos(
        base_name: String,
        name: String,
        data: KhronosValue,
    ) -> Self {
        Self {
            base_name,
            name,
            data: InnerEventData::Khronos(data),
        }
    }
}

impl CreateEvent {
    /// Returns the base name of the event
    pub fn base_name(&self) -> &str {
        &self.base_name
    }

    /// Returns the name (NOT the base name) of the event
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// An `Event` is an object that can be passed to provide data to a Lua script.
#[derive(Clone)]
pub struct Event {
    tab: LuaTable,
}

impl Event {
    /// Converts the `CreateEvent` into an `Event`
    pub fn from_create_event(lua: &mluau::Lua, ce: CreateEvent) -> Result<Event, mluau::Error> {
        let tab = lua.create_table()?;
        tab.set("base_name", ce.base_name.clone())?;
        tab.set("name", ce.name.clone())?;
        tab.set(
            "data",
            match &ce.data {
                InnerEventData::Json(ref value) => {
                    lua.to_value_with(value, LUA_SERIALIZE_OPTIONS)?
                },
                InnerEventData::RawValue(raw_value) => {
                    let value: serde_json::Value = serde_json::from_str(raw_value.get())
                        .map_err(|e| LuaError::external(e))?;
                    lua.to_value_with(&value, LUA_SERIALIZE_OPTIONS)?
                },
                InnerEventData::Khronos(khronos_value) => {
                    khronos_value.into_lua_from_ref(lua, 0)?
                },
            },
        )?;
        tab.set_readonly(true);
        Ok(Event { tab })
    }

    pub fn from_create_event_with_runtime(
        runtime: &KhronosRuntime,
        ce: CreateEvent
    ) -> Result<Event, mluau::Error> {
        let Some(ref lua) = *runtime.lua() else {
            return Err(LuaError::external("Runtime Lua instance not available"));
        };
        Event::from_create_event(lua, ce)
    }
}

impl IntoLua for Event {
    fn into_lua(self, _lua: &Lua) -> LuaResult<LuaValue> {
        Ok(LuaValue::Table(self.tab)) 
    }
}
