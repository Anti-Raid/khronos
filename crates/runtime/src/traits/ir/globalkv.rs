use mluau::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{core::datetime::DateTime, primitives::{lazy::Lazy, opaque::Opaque}};

#[derive(Debug, Serialize, Deserialize)]
/// A global key-value entry that can be viewed by all guilds
/// 
/// Unlike normal key-values, these are not scoped to a specific guild or tenant,
/// are immutable (new versions must be created, updates not allowed) and have both
/// a public metadata and potentially private value. Only staff may create global kv's that
/// have a price attached to them.
/// 
/// These are primarily used for things like the template shop but may be used for other
/// things as well in the future beyond template shop as well such as global lists.
pub struct GlobalKv {
    pub key: String,
    pub version: i32,
    pub owner_id: String,
    pub owner_type: String,
    pub price: Option<i64>, // will only be set for shop items, otherwise None
    pub short: String, // short description for the key-value.
    pub public_metadata: serde_json::Value, // public metadata about the key-value
    pub scope: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
    pub public_data: bool,
    pub review_state: String,
    pub long: Option<String>, // long description for the key-value.
    pub data: serde_json::Value, // the actual value of the key-value, may be private
}

impl IntoLua for GlobalKv {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        table.set("key", self.key)?;
        table.set("version", self.version)?;
        table.set("owner_id", self.owner_id)?;
        table.set("owner_type", self.owner_type)?;
        table.set("price", match self.price {
            Some(p) => LuaValue::Integer(p),
            None => LuaValue::Nil,
        })?;
        table.set("short", self.short)?;
        table.set("long", match self.long {
            Some(l) => LuaValue::String(lua.create_string(&l)?),
            None => LuaValue::Nil,
        })?;
        table.set("public_metadata", lua.to_value(&self.public_metadata)?)?;
        table.set("scope", self.scope)?;
        table.set("created_at", DateTime::from_utc(self.created_at))?;
        table.set("last_updated_at", DateTime::from_utc(self.last_updated_at))?;
        table.set("public_data", self.public_data)?;
        table.set("review_state", self.review_state)?;
        if self.data.is_null() {
            table.set("data", LuaValue::Nil)?;
        } else {
            let data = if self.price.is_some() || !self.public_data {
                lua.create_userdata(Opaque::new(self.data))?
            } else {
                lua.create_userdata(Lazy::new(self.data))?
            };
            table.set("data", data)?;
        }
        table.set_readonly(true); // We want KvRecords to be immutable
        Ok(LuaValue::Table(table))
    }
}

/// A global key-value entry that can be viewed by all guilds
/// 
/// Unlike normal key-values, these are not scoped to a specific guild or tenant,
/// are immutable (new versions must be created, updates not allowed) and have both
/// a public metadata and potentially private value. Only staff may create global kv's that
/// have a price attached to them.
/// 
/// These are primarily used for things like the template shop but may be used for other
/// things as well in the future beyond template shop as well such as global lists.
/// 
/// NOTE: Global KV's created publicly cannot have a price associated to them for legal reasons.
/// Only staff may create priced global KV's.
/// NOTE 2: All Global KV's undergo staff review before being made available. When this occurs,
/// review state will be updated accordingly from 'pending' to 'approved' or otherwise if rejected.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGlobalKv {
    pub key: String,
    pub version: i32,
    pub short: String, // short description for the key-value.
    pub public_metadata: serde_json::Value, // public metadata about the key-value
    pub scope: String,
    pub public_data: bool,
    pub long: Option<String>, // long description for the key-value.
    pub data: serde_json::Value, // the actual value of the key-value, may be private
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum AttachResult {
    PurchaseRequired { url: String },
    Ok(()),
}