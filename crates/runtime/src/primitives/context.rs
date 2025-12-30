use crate::plugins::{antiraid, antiraid::LUA_SERIALIZE_OPTIONS};
use crate::primitives::event::CreateEvent;
use crate::traits::context::{KhronosContext, KhronosValueWith, Limitations};
use dapi::controller::DiscordProvider;
use mluau::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

enum EventState {
    /// The event has not been consumed yet
    CreateEvent(CreateEvent),
    /// The event has been consumed and is stored as a LuaValue
    LuaValue(LuaValue),
}

impl Default for EventState {
    fn default() -> Self {
        Self::LuaValue(LuaValue::Nil)
    }
}

#[derive(Clone)]
pub struct TemplateContext<T: KhronosContext> {
    pub context: T,

    /// The current limitations for the context.
    ///
    /// This will (for the outermost context) be the same as `context.limitations()`.
    ///
    /// For subcontexts (created with `ctx:withlimits`), this will be a subset of the outer limitations.
    pub limitations: Rc<Limitations>,

    /// The cached serialized value of the current user
    current_discord_user: Rc<RefCell<Option<LuaValue>>>,

    /// Store table
    store_table: LuaTable,

    event: Rc<RefCell<EventState>>,

    /// Cached plugin data
    pub(crate) cached_plugin_data: Rc<RefCell<HashMap<String, LuaValue>>>,
}

impl<T: KhronosContext> std::fmt::Debug for TemplateContext<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateContext")
            .field("store_table", &self.store_table)
            .finish()
    }
}

impl<T: KhronosContext> TemplateContext<T> {
    pub(crate) fn new(store_table: LuaTable, context: T, event: CreateEvent) -> LuaResult<Self> {
        Ok(Self {
            limitations: Rc::new(context.limitations()),
            context,
            store_table,
            current_discord_user: Rc::default(),
            cached_plugin_data: Rc::default(), // Safety note: the cached plugin data must be reset for subcontexts to avoid privilege escalation across subcontexts
            event: Rc::new(RefCell::new(EventState::CreateEvent(event))),
        })
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

        let Some(dp) = self.context.discord_provider() else {
            return Err(LuaError::external("Current user not found"));
        };

        let v = lua.to_value_with(&dp.current_user(), LUA_SERIALIZE_OPTIONS)?;

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
        fields.add_field_method_get("Discord", |lua, this| {
            this.get_plugin(lua, "Discord", antiraid::discord::init_plugin)
        });

        fields.add_field_method_get("ImageCaptcha", |lua, this| {
            this.get_plugin(lua, "ImageCaptcha", antiraid::img_captcha::init_plugin)
        });

        fields.add_field_method_get("KV", |lua, this| {
            this.get_plugin(lua, "KV", antiraid::kv::init_plugin)
        });

        fields.add_field_method_get("ObjectStorage", |lua, this| {
            this.get_plugin(lua, "ObjectStorage", antiraid::objectstorage::init_plugin)
        });

        fields.add_field_method_get("HTTPClient", |lua, this| {
            this.get_plugin(lua, "HTTPClient", antiraid::httpclient::init_plugin)
        });

        fields.add_field_method_get("HTTPServer", |lua, this| {
            this.get_plugin(lua, "HTTPServer", antiraid::httpserver::init_plugin)
        });

        fields.add_field_method_get("Runtime", |lua, this| {
            this.get_plugin(lua, "Runtime", antiraid::runtime::init_plugin)
        });

        let mut available_extra_plugins = Vec::new();
        for (plugin_name, plugin_init) in T::extra_plugins() {
            available_extra_plugins.push(plugin_name.clone());
            fields.add_field_method_get(plugin_name.clone(), move |lua, this| {
                let plugin_func = |lua: &Lua, this: &Self| plugin_init(lua, this);
                this.get_plugin(lua, &plugin_name, plugin_func)
            });
        }

        fields.add_field_method_get("available_extra_plugins", move |_lua, _this| {
            Ok(available_extra_plugins.clone())
        });

        // Fields
        fields.add_field_method_get("store", |_, this| Ok(this.store_table.clone()));

        fields.add_field_method_get("guild_id", |lua, this| {
            let v = lua.to_value_with(&this.context.guild_id(), LUA_SERIALIZE_OPTIONS)?;

            Ok(v)
        });

        fields.add_field_method_get("allowed_caps", |lua, this| {
            let v = lua.to_value_with(&this.limitations.capabilities, LUA_SERIALIZE_OPTIONS)?;

            Ok(v)
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

        methods.add_method("event", |lua, this, _: ()| {
            let mut event_state = this
                .event
                .try_borrow_mut()
                .map_err(|e| LuaError::external(e.to_string()))?;

            match std::mem::replace(&mut *event_state, EventState::LuaValue(LuaValue::Nil)) {
                EventState::CreateEvent(event) => {
                    let lua_value = event.into_lua(lua)?;
                    *event_state = EventState::LuaValue(lua_value.clone());
                    Ok(lua_value)
                }
                EventState::LuaValue(lua_value) => {
                    *event_state = EventState::LuaValue(lua_value.clone());
                    Ok(lua_value)
                }
            }
        }); 

        methods.add_method("has_cap", |_, this, cap: String| {
            Ok(this.limitations.has_cap(&cap))
        });

        methods.add_method("has_any_cap", |_, this, caps: Vec<String>| {
            Ok(this.limitations.has_any_cap(&caps))
        });

        methods.add_method("with", |lua, this, with: LuaValue| {
            let with: KhronosValueWith = lua.from_value(with)?;

            let limits = Limitations::new(with.capabilities);

            // Ensure that the new limitations are a subset of the current limitations
            limits
                .subset_of(&this.limitations)
                .map_err(mluau::Error::external)?;

            // Create a new context with the given limitations
            let new_context = TemplateContext {
                limitations: Rc::new(limits),
                context: this.context.clone(),
                store_table: this.store_table.clone(),
                current_discord_user: this.current_discord_user.clone(),
                cached_plugin_data: this.cached_plugin_data.clone(),
                event: match with.event {
                    Some(e) => Rc::new(RefCell::new(EventState::CreateEvent(e))),
                    None => this.event.clone(),
                },
            };

            Ok(new_context)
        });
    }

    #[cfg(feature = "repl")]
    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
    }
}
