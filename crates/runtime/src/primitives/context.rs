use crate::traits::context::KhronosContext;
use mlua::prelude::*;
use std::cell::RefCell;

pub struct TemplateContext<T: KhronosContext> {
    pub context: T,

    /// The cached serialized value of the data
    cached_data: RefCell<Option<LuaValue>>,

    /// The cached serialized value of the current user
    current_discord_user: RefCell<Option<LuaValue>>,
}

impl<T: KhronosContext> TemplateContext<T> {
    pub fn new(context: T) -> Self {
        Self {
            context,
            cached_data: RefCell::default(),
            current_discord_user: RefCell::default(),
        }
    }
}

pub type TemplateContextRef<T> = LuaUserDataRef<TemplateContext<T>>;

/*
            // Check for cached serialized data
            let mut cached_data = this
                .current_discord_user
                .try_borrow_mut()
                .map_err(|e| LuaError::external(e.to_string()))?;

            if let Some(v) = cached_data.as_ref() {
                return Ok(v.clone());
            }

            let v = lua.to_value(&this.context.data())?;

            *cached_data = Some(v.clone());

            Ok(v)
*/

impl<T: KhronosContext> LuaUserData for TemplateContext<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("data", |lua, this| {
            // Check for cached serialized data
            let mut cached_data = this
                .cached_data
                .try_borrow_mut()
                .map_err(|e| LuaError::external(e.to_string()))?;

            if let Some(v) = cached_data.as_ref() {
                return Ok(v.clone());
            }

            let v = lua.to_value(&this.context.data())?;

            *cached_data = Some(v.clone());

            Ok(v)
        });

        fields.add_field_method_get("guild_id", |lua, this| {
            let v = lua.to_value(&this.context.guild_id())?;

            Ok(v)
        });

        fields.add_field_method_get("owner_guild_id", |lua, this| {
            let v = lua.to_value(&this.context.owner_guild_id())?;

            Ok(v)
        });

        fields.add_field_method_get("allowed_caps", |lua, this| {
            let v = lua.to_value(this.context.allowed_caps())?;

            Ok(v)
        });

        fields.add_field_method_get("current_user", |lua, this| {
            // Check for cached serialized data
            let mut cached_data = this
                .current_discord_user
                .try_borrow_mut()
                .map_err(|e| LuaError::external(e.to_string()))?;

            if let Some(v) = cached_data.as_ref() {
                return Ok(v.clone());
            }

            let v = lua.to_value(&this.context.data())?;

            *cached_data = Some(v.clone());

            Ok(v)
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("has_cap", |_, this, cap: String| {
            Ok(this.context.has_cap(&cap))
        });
    }
}
