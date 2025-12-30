use mluau::prelude::*;
use crate::{plugins::antiraid::LUA_SERIALIZE_OPTIONS, utils::khronos_value::KhronosValue};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum InnerEventData {
    /// The inner data of the object
    Json(serde_json::Value),
    RawValue(Box<serde_json::value::RawValue>),
    KhronosValue(KhronosValue)
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

    /// Create a new Event given a KhronosValue
    pub fn new_khronos_value(
        name: String,
        data: KhronosValue,
    ) -> Self {
        Self {
            name,
            data: InnerEventData::KhronosValue(data),
        }
    }
}

impl IntoLua for CreateEvent {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
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
                InnerEventData::KhronosValue(kv) => {
                    kv.into_lua(lua)?
                }
            },
        )?;
        tab.set_readonly(true);
        Ok(LuaValue::Table(tab))
    }
}
