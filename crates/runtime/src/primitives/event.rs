use mluau::prelude::*;
use crate::{primitives::LUA_SERIALIZE_OPTIONS, utils::khronos_value::KhronosValue};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InnerEventData {
    /// The inner data of the object
    Json(serde_json::Value),
    RawValue(Box<serde_json::value::RawValue>),
    KhronosValue(KhronosValue)
}

/// An `CreateEvent` is a/an thread-safe object that can be used to create a Event
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateEvent {
    /// The name of the event
    name: String,
    /// The authorized author of the event
    author: Option<String>,
    /// The inner data of the object
    data: InnerEventData,
}

impl CreateEvent {
    /// Create a new Event
    pub fn new(
        name: String,
        author: Option<String>,
        data: serde_json::Value,
    ) -> Self {
        Self {
            name,
            author,
            data: InnerEventData::Json(data),
        }
    }

    /// Create a new Event given a raw value
    pub fn new_raw_value(
        name: String,
        author: Option<String>,
        data: Box<serde_json::value::RawValue>,
    ) -> Self {
        Self {
            name,
            author,
            data: InnerEventData::RawValue(data),
        }
    }

    /// Create a new Event given a KhronosValue
    pub fn new_khronos_value(
        name: String,
        author: Option<String>,
        data: KhronosValue,
    ) -> Self {
        Self {
            name,
            author,
            data: InnerEventData::KhronosValue(data),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    pub fn extract(self) -> (String, Option<String>, InnerEventData) {
        (self.name, self.author, self.data)
    }
}

impl FromLua for CreateEvent {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(t) => t,
            _ => return Err(LuaError::FromLuaConversionError { from: value.type_name(), to: "KhronosValueWith".to_string(), message: Some("Expected a table".to_string()) }),
        };

        let name: String = table.get("name")?;
        let author: Option<String> = table.get("author")?;
        let data: KhronosValue = table.get("data")?;        

        Ok(CreateEvent {
            name,
            author,
            data: InnerEventData::KhronosValue(data),
        })
    }
}

impl IntoLua for CreateEvent {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let tab = lua.create_table()?;
        tab.set("name", self.name.as_str())?;
        match self.author {
            Some(author) => tab.set("author", author.as_str())?,
            None => {},
        }
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
