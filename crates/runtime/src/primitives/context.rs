use crate::plugins::{antiraid, antiraid::LUA_SERIALIZE_OPTIONS};
use crate::traits::context::{KhronosContext, Limitations};
use crate::utils::khronos_value::KhronosValue;
use mluau::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::create_userdata_iterator_with_fields;

pub struct TemplateContext<T: KhronosContext> {
    pub context: T,

    /// The current limitations for the context.
    ///
    /// This will (for the outermost context) be the same as `context.limitations()`.
    ///
    /// For subcontexts (created with `ctx:withlimits`), this will be a subset of the outer limitations.
    pub limitations: Rc<Limitations>,

    /// The cached serialized value of the data
    cached_data: RefCell<Option<LuaValue>>,

    /// The cached serialized value of the current user
    current_discord_user: RefCell<Option<LuaValue>>,

    /// Store table
    store_table: LuaTable,

    /// Cached plugin data
    pub(crate) cached_plugin_data: RefCell<HashMap<String, LuaValue>>,
}

impl<T: KhronosContext> TemplateContext<T> {
    pub fn new(lua: &Lua, context: T) -> LuaResult<Self> {
        let store = lua
            .app_data_ref::<crate::rt::runtime::RuntimeGlobalTable>()
            .ok_or(mluau::Error::RuntimeError(
                "No runtime global table found".to_string(),
            ))?;

        Ok(Self {
            limitations: Rc::new(context.limitations()),
            context,
            store_table: store.0.clone(),
            cached_data: RefCell::default(),
            current_discord_user: RefCell::default(),
            cached_plugin_data: RefCell::new(HashMap::new()),
        })
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

        let v = lua.to_value_with(&self.context.data(), LUA_SERIALIZE_OPTIONS)?;

        *cached_data = Some(v.clone());

        Ok(v)
    }

    fn get_cached_current_user(&self, lua: &Lua) -> LuaResult<LuaValue> {
        // Check for cached serialized data
        let mut cached_data = self
            .current_discord_user
            .try_borrow_mut()
            .map_err(|e| LuaError::external(e.to_string()))?;

        if let Some(v) = cached_data.as_ref() {
            return Ok(v.clone());
        }

        let v = lua.to_value_with(&self.context.current_user(), LUA_SERIALIZE_OPTIONS)?;

        *cached_data = Some(v.clone());

        Ok(v)
    }

    /// Gets a plugin from cache or runs 'f' to get it
    fn get_plugin<F, V>(&self, lua: &Lua, plugin_name: &str, f: F) -> LuaResult<LuaValue>
    where
        F: FnOnce(&Lua, &Self) -> LuaResult<V>,
        V: IntoLua,
    {
        let mut cached_plugin_data = self
            .cached_plugin_data
            .try_borrow_mut()
            .map_err(|e| LuaError::external(e.to_string()))?;

        if let Some(v) = cached_plugin_data.get(plugin_name) {
            return Ok(v.clone());
        }

        let v = f(lua, self)?.into_lua(lua)?;

        cached_plugin_data.insert(plugin_name.to_string(), v.clone());

        Ok(v)
    }
}

impl<T: KhronosContext> LuaUserData for TemplateContext<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // Plugins
        fields.add_field_method_get("DataStores", |lua, this| {
            this.get_plugin(lua, "DataStores", antiraid::datastores::init_plugin)
        });

        fields.add_field_method_get("Discord", |lua, this| {
            this.get_plugin(lua, "Discord", antiraid::discord::init_plugin)
        });

        fields.add_field_method_get("ImageCaptcha", |lua, this| {
            this.get_plugin(lua, "ImageCaptcha", antiraid::img_captcha::init_plugin)
        });

        fields.add_field_method_get("KV", |lua, this| {
            this.get_plugin(lua, "KV", antiraid::kv::init_plugin)
        });

        fields.add_field_method_get("UnscopedKV", |lua, this| {
            if !this.limitations.has_cap("kv.meta:unscoped_allowed") {
                return Err(LuaError::external(
                    "The kv.meta:unscoped_allowed capability is required to create an UnscopedKV",
                ));
            }

            this.get_plugin(lua, "UnscopedKV", antiraid::kv::init_plugin_unscoped)
        });

        fields.add_field_method_get("ObjectStorage", |lua, this| {
            this.get_plugin(lua, "ObjectStorage", antiraid::objectstorage::init_plugin)
        });

        fields.add_field_method_get("HTTPClient", |lua, this| {
            this.get_plugin(lua, "HTTPClient", antiraid::httpclient::init_plugin)
        });

        // Fields
        fields.add_field_method_get("store", |_, this| Ok(this.store_table.clone()));

        fields.add_field_method_get("data", |lua, this| {
            let data = this.get_cached_data(lua)?;
            Ok(data)
        });

        fields.add_field_method_get("guild_id", |lua, this| {
            let v = lua.to_value_with(&this.context.guild_id(), LUA_SERIALIZE_OPTIONS)?;

            Ok(v)
        });

        fields.add_field_method_get("owner_guild_id", |lua, this| {
            let v = lua.to_value_with(&this.context.owner_guild_id(), LUA_SERIALIZE_OPTIONS)?;

            Ok(v)
        });

        fields.add_field_method_get("allowed_caps", |lua, this| {
            let v = lua.to_value_with(&this.limitations.capabilities, LUA_SERIALIZE_OPTIONS)?;

            Ok(v)
        });

        fields.add_field_method_get("template_name", |_lua, this| {
            Ok(this.context.template_name())
        });

        fields.add_field_method_get("current_user", |lua, this| {
            let v = this.get_cached_current_user(lua)?;

            Ok(v)
        });

        fields.add_field_method_get("memory_limit", |_lua, this| Ok(this.context.memory_limit()));

        fields.add_meta_field(LuaMetaMethod::Type, "TemplateContext".to_string());
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, _, _: ()| Ok("TemplateContext"));

        methods.add_method("has_cap", |_, this, cap: String| {
            Ok(this.limitations.has_cap(&cap))
        });

        methods.add_method("has_any_cap", |_, this, caps: Vec<String>| {
            Ok(this.limitations.has_any_cap(&caps))
        });

        methods.add_method("withlimits", |_lua, this, limits: KhronosValue| {
            let limits: Limitations = limits.try_into().map_err(|e| {
                mluau::Error::external(format!("Failed to convert LuaValue to Limitations: {e}"))
            })?;

            // Ensure that the new limitations are a subset of the current limitations
            limits
                .subset_of(&this.limitations)
                .map_err(mluau::Error::external)?;

            // Create a new context with the given limitations
            let new_context = TemplateContext {
                limitations: Rc::new(limits),
                context: this.context.clone(),
                store_table: this.store_table.clone(),
                cached_data: RefCell::default(),
                current_discord_user: RefCell::default(),
                cached_plugin_data: RefCell::new(HashMap::new()),
            };

            Ok(new_context)
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<TemplateContext<T>>() {
                return Err(mluau::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "DataStores",
                    "Discord",
                    "ImageCaptcha",
                    "KV",
                    "Lockdowns",
                    "ObjectStorage",
                    "Pages",
                    "ScheduledExecution",
                    "UserInfo",
                    // Fields (raw)
                    "data",
                    "guild_id",
                    "owner_guild_id",
                    "allowed_caps",
                    "current_user",
                    // Methods
                    "has_cap",
                    "has_any_cap",
                    "withlimits",
                ],
            )
        });
    }
}
