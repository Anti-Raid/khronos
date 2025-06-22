use crate::{
    plugins::antiraid::LUA_SERIALIZE_OPTIONS,
    traits::{context::KhronosContext, lockdownprovider::LockdownProvider},
};
use mlua::prelude::*;
use std::rc::Rc;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use crate::{
    core::datetime::DateTime, primitives::create_userdata_iterator_with_fields,
};

pub struct CreateLockdownMode(pub Box<(dyn lockdowns::CreateLockdownMode + 'static)>);

impl Clone for CreateLockdownMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl LuaUserData for CreateLockdownMode {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("syntax", |_, this| {
            let syntax = this.0.syntax();
            Ok(syntax.to_string())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("to_lockdown_mode", |_, this, string_form: String| {
            let lockdown_mode = this
                .0
                .to_lockdown_mode(&string_form)
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(lockdown_mode.map(LockdownMode))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<CreateLockdownMode>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "syntax",
                    // Methods
                    "to_lockdown_mode",
                ],
            )
        });
    }
}

pub struct LockdownMode(pub Box<(dyn lockdowns::LockdownMode + 'static)>);

impl Clone for LockdownMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl LuaUserData for LockdownMode {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("creator", |_, this| {
            let creator = this.0.creator();
            Ok(CreateLockdownMode(creator))
        });

        fields.add_field_method_get("string_form", |_, this| {
            let string_form = this.0.string_form();
            Ok(string_form)
        });

        fields.add_field_method_get("specificity", |_, this| {
            let specificity = this.0.specificity();
            Ok(specificity)
        });
    }
}

pub struct Lockdown(lockdowns::Lockdown);

impl From<lockdowns::Lockdown> for Lockdown {
    fn from(value: lockdowns::Lockdown) -> Self {
        Self(value)
    }
}

impl LuaUserData for Lockdown {
    /*
    pub id: uuid::Uuid,
    pub reason: String,
    pub r#type: Box<dyn LockdownMode>,
    pub data: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
     */

    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.0.id.to_string()));

        fields.add_field_method_get("reason", |_, this| {
            let reason = this.0.reason.clone();
            Ok(reason)
        });

        fields.add_field_method_get("type", |_, this| {
            let r#type = this.0.r#type.clone();
            Ok(LockdownMode(r#type))
        });

        fields.add_field_method_get("data", |lua, this| {
            let lua_value = lua
                .to_value(&this.0.data)
                .map_err(|e| LuaError::external(format!("Error while serializing data: {}", e)))?;

            Ok(lua_value)
        });

        fields.add_field_method_get("created_at", |_, this| {
            let created_at = this.0.created_at;
            Ok(DateTime::from_utc(created_at))
        });
    }

    fn add_methods<F: LuaUserDataMethods<Self>>(methods: &mut F) {
        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<Lockdown>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "id",
                    "reason",
                    "type",
                    "data",
                    "created_at",
                ],
            )
        });
    }
}

pub struct LockdownSet<T: KhronosContext> {
    pub context: T,
    pub lockdown_provider: T::LockdownProvider,
    pub lockdown_set: lockdowns::LockdownSet<T::LockdownDataStore>,
}

impl<T: KhronosContext> LockdownSet<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.context.has_cap(&format!("lockdown:{}", action)) {
            return Err(LuaError::runtime(
                "Lockdown action is not allowed in this template context",
            ));
        }

        self.lockdown_provider
            .attempt_action(&action)
            .map_err(|e| LuaError::external(e.to_string()))?;

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for LockdownSet<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("lockdowns", |_, this| {
            let lockdowns = this
                .lockdown_set
                .lockdowns()
                .iter()
                .map(|l| Lockdown::from(l.clone()))
                .collect::<Vec<_>>();

            Ok(lockdowns)
        });

        fields.add_field_method_get("settings", |lua, this| {
            let settings = lua
                .to_value_with(&this.lockdown_set.settings(), LUA_SERIALIZE_OPTIONS)
                .map_err(|e| {
                    LuaError::external(format!("Error while serializing settings: {}", e))
                })?;

            Ok(settings)
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(LuaMetaMethod::Len, |_, this, _: ()| {
            let len = this.lockdown_set.lockdowns().len();

            Ok(len)
        });

        // Sorts the lockdowns by specificity in descending order
        methods.add_method_mut("sort", |_, this, _: ()| {
            this.lockdown_set.sort();

            Ok(())
        });

        methods.add_scheduler_async_method_mut(
            "apply",
            async move |_lua, mut this, (lockdown_type, reason): (LuaUserDataRef<LockdownMode>, String)| {            
                this.check_action(lockdown_type.0.string_form())?;

                match this
                    .lockdown_set
                    .apply(lockdown_type.0.clone(), &reason)
                    .await {
                    Ok(lockdown_id) => Ok(LockdownAddStatus::Ok(lockdown_id)),
                    Err(e) => match e {
                        lockdowns::LockdownError::LockdownTestFailed(e) => {
                            Ok(LockdownAddStatus::LockdownTestFailed(LockdownTestResult(
                                Rc::new(e),
                                std::marker::PhantomData::<T>
                            )))
                        }
                        lockdowns::LockdownError::Error(e) => {
                            Ok(LockdownAddStatus::Error(e.to_string()))
                        },
                    }
                }
            },
        );

        methods.add_scheduler_async_method_mut(
            "remove",
            async move |_lua, mut this, id: String| {
                this.check_action("remove".to_string())?;

                let id = uuid::Uuid::parse_str(&id)
                    .map_err(|e| LuaError::external(format!("Invalid UUID: {}", e)))?;

                match this.lockdown_set
                    .remove(id)
                    .await {
                    Ok(()) => Ok(LockdownRemoveStatus::Ok),
                    Err(e) => match e {
                        lockdowns::LockdownError::LockdownTestFailed(e) => {
                            Ok(LockdownRemoveStatus::LockdownTestFailed(LockdownTestResult(
                                Rc::new(e),
                                std::marker::PhantomData::<T>,
                            )))
                        }
                        lockdowns::LockdownError::Error(e) => {
                            Ok(LockdownRemoveStatus::Error(e.to_string()))
                        },
                    }
                }
            },
        );

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<LockdownSet<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "lockdowns",
                    "settings",
                    // Methods
                    "apply",
                    "remove",
                    "sort",
                ],
            )
        });
    }
}

pub enum LockdownAddStatus<T: KhronosContext> {
    Ok(uuid::Uuid),
    Error(String),
    LockdownTestFailed(LockdownTestResult<T>),
}

impl<T: KhronosContext> LuaUserData for LockdownAddStatus<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "LockdownAddStatus");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Define __index metamethod
        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: LuaValue| {
            let key = match key {
                LuaValue::String(key) => key.to_string_lossy(),
                _ => {
                    return Ok(LuaValue::Nil);
                }
            };

            let v = match key.as_str() {
                "ok" => match this {
                    LockdownAddStatus::Ok(_) => LuaValue::Boolean(true),
                    LockdownAddStatus::Error(_) => LuaValue::Boolean(false),
                    LockdownAddStatus::LockdownTestFailed(_) => LuaValue::Boolean(false),
                },
                "type" => match this {
                    LockdownAddStatus::Ok(_) => LuaValue::String(lua.create_string("Ok")?),
                    LockdownAddStatus::Error(_) => LuaValue::String(lua.create_string("Error")?),
                    LockdownAddStatus::LockdownTestFailed(_) => {
                        LuaValue::String(lua.create_string("LockdownTestFailed")?)
                    }
                },
                "id" => match this {
                    LockdownAddStatus::Ok(id) => {
                        LuaValue::String(lua.create_string(&id.to_string())?)
                    }
                    LockdownAddStatus::Error(_) => LuaValue::Nil,
                    LockdownAddStatus::LockdownTestFailed(_) => LuaValue::Nil,
                },
                "test_result" => match this {
                    LockdownAddStatus::Ok(_) => LuaValue::Nil,
                    LockdownAddStatus::Error(_) => LuaValue::Nil,
                    LockdownAddStatus::LockdownTestFailed(e) => {
                        let e = e.clone();
                        e.into_lua(lua)?
                    }
                },
                "error" => match this {
                    LockdownAddStatus::Ok(_) => LuaValue::Nil,
                    LockdownAddStatus::Error(e) => LuaValue::String(lua.create_string(e)?),
                    LockdownAddStatus::LockdownTestFailed(e) => {
                        LuaValue::String(lua.create_string(&e.0.display_error())?)
                    }
                },
                _ => mlua::Value::Nil,
            };

            Ok(v)
        });

        // Iter
        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<LockdownAddStatus<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "ok",
                    "type",
                    "id",
                    "error",
                    "test_result",
                    // Methods
                ],
            )
        });

        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            let s = match this {
                LockdownAddStatus::Ok(id) => id.to_string(),
                LockdownAddStatus::Error(e) => e.to_string(),
                LockdownAddStatus::LockdownTestFailed(e) => e.0.display_error(),
            };

            Ok(s)
        });
    }
}

pub enum LockdownRemoveStatus<T: KhronosContext> {
    Ok,
    Error(String),
    LockdownTestFailed(LockdownTestResult<T>),
}

impl<T: KhronosContext> LuaUserData for LockdownRemoveStatus<T> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Define __index metamethod
        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: LuaValue| {
            let key = match key {
                LuaValue::String(key) => key.to_string_lossy(),
                _ => {
                    return Ok(LuaValue::Nil);
                }
            };

            let v = match key.as_str() {
                "ok" => match this {
                    LockdownRemoveStatus::Ok => LuaValue::Boolean(true),
                    LockdownRemoveStatus::Error(_) => LuaValue::Boolean(false),
                    LockdownRemoveStatus::LockdownTestFailed(_) => LuaValue::Boolean(false),
                },
                "type" => match this {
                    LockdownRemoveStatus::Ok => LuaValue::String(lua.create_string("Ok")?),
                    LockdownRemoveStatus::Error(_) => LuaValue::String(lua.create_string("Error")?),
                    LockdownRemoveStatus::LockdownTestFailed(_) => {
                        LuaValue::String(lua.create_string("LockdownTestFailed")?)
                    }
                },
                "test_result" => match this {
                    LockdownRemoveStatus::Ok => LuaValue::Nil,
                    LockdownRemoveStatus::Error(_) => LuaValue::Nil,
                    LockdownRemoveStatus::LockdownTestFailed(e) => {
                        let e = e.clone();
                        e.into_lua(lua)?
                    }
                },
                "error" => match this {
                    LockdownRemoveStatus::Ok => LuaValue::Nil,
                    LockdownRemoveStatus::Error(e) => LuaValue::String(lua.create_string(e)?),
                    LockdownRemoveStatus::LockdownTestFailed(e) => {
                        LuaValue::String(lua.create_string(&e.0.display_error())?)
                    }
                },
                _ => mlua::Value::Nil,
            };

            Ok(v)
        });

        // Iter
        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<LockdownRemoveStatus<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "ok",
                    "type",
                    "error",
                    "test_result",
                    // Methods
                ],
            )
        });

        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            let s = match this {
                LockdownRemoveStatus::Ok => "Ok".to_string(),
                LockdownRemoveStatus::Error(e) => e.to_string(),
                LockdownRemoveStatus::LockdownTestFailed(e) => e.0.display_error(),
            };

            Ok(s)
        });
    }
}

#[derive(Clone)]
pub struct LockdownTestResult<T: KhronosContext>(
    Rc<lockdowns::LockdownTestResult>,
    std::marker::PhantomData<T>,
);

impl<T: KhronosContext> LuaUserData for LockdownTestResult<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("can_apply_perfectly", |_, this| {
            let can_apply_perfectly = this.0.can_apply_perfectly();
            Ok(can_apply_perfectly)
        });

        fields.add_field_method_get("role_changes_needed", |lua, this| {
            lua.to_value_with(&this.0.role_changes_needed, LUA_SERIALIZE_OPTIONS)
                .map_err(|e| {
                    LuaError::external(format!(
                        "Error while serializing role changes needed: {}",
                        e
                    ))
                })
        });

        fields.add_field_method_get("other_changes_needed", |lua, this| {
            lua.to_value_with(&this.0.other_changes_needed, LUA_SERIALIZE_OPTIONS)
                .map_err(|e| {
                    LuaError::external(format!(
                        "Error while serializing other changes needed: {}",
                        e
                    ))
                })
        });

        fields.add_meta_field(LuaMetaMethod::Type, "LockdownTestResult");
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("display_error", |_, this, _: ()| {
            let s = this.0.display_error();
            Ok(s)
        });

        methods.add_scheduler_async_method("display_changeset", async move |_, this, lockdown_set: LuaAnyUserData| {
            let mut lockdown_set = lockdown_set
            .borrow_mut::<LockdownSet<T>>()
            .map_err(|_| LuaError::external("Failed to lock access to lockdown test result"))?;

            let partial_guild = lockdown_set.lockdown_set.partial_guild().await
            .map_err(|e| LuaError::external(format!("Failed to get partial guild: {}", e)))?;

            let changeset = this.0.display_changeset(partial_guild);

            Ok(changeset)
        });

        methods.add_scheduler_async_method("try_auto_fix", async move |_, this, lockdown_set: LuaAnyUserData| {
            let mut lockdown_set = lockdown_set
            .borrow_mut::<LockdownSet<T>>()
            .map_err(|_| LuaError::external("Failed to lock access to lockdown test result"))?;

            let lockdown_provider = lockdown_set.lockdown_provider.clone();
            let http = lockdown_provider.serenity_http();
            let partial_guild = lockdown_set.lockdown_set.partial_guild_mut().await
            .map_err(|e| LuaError::external(format!("Failed to get partial guild: {}", e)))?;

            this.0.try_auto_fix(http, partial_guild)
            .await
            .map_err(|e| LuaError::external(format!("Failed to apply changes: {}", e)))?;

            Ok(())
        });

        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            let s = this.0.display_error();
            Ok(s)
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<LockdownTestResult<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "can_apply_perfectly",
                    "role_changes_needed",
                    "other_changes_needed",
                    // Methods
                    "display_changeset",
                ],
            )
        });
    }
}
