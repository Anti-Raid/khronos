use crate::traits::context::KhronosContext;
use mlua::prelude::*;
use std::cell::RefCell;

use super::create_userdata_iterator_with_fields;

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

    fn get_cached_data(&self, lua: &Lua) -> LuaResult<LuaValue> {
        // Check for cached serialized data
        let mut cached_data = self
            .cached_data
            .try_borrow_mut()
            .map_err(|e| LuaError::external(e.to_string()))?;

        if let Some(v) = cached_data.as_ref() {
            return Ok(v.clone());
        }

        let v = lua.to_value(&self.context.data())?;

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

        let v = lua.to_value(&self.context.current_user())?;

        *cached_data = Some(v.clone());

        Ok(v)
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
            let data = this.get_cached_data(lua)?;
            Ok(data)
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
            let v = this.get_cached_current_user(lua)?;

            Ok(v)
        });

        fields.add_meta_field(LuaMetaMethod::Type, "TemplateContext".to_string());
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, _, _: ()| Ok("TemplateContext"));

        methods.add_method("has_cap", |_, this, cap: String| {
            Ok(this.context.has_cap(&cap))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<TemplateContext<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "data",
                    "guild_id",
                    "owner_guild_id",
                    "allowed_caps",
                    "current_user",
                    // Methods
                    "has_cap",
                ],
            )
        });
    }
}
