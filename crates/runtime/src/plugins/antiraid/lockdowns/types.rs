use crate::{
    lua_promise,
    traits::{context::KhronosContext, lockdownprovider::LockdownProvider},
};
use mlua::prelude::*;
use std::rc::Rc;

use crate::{
    plugins::antiraid::datetime::DateTime, primitives::create_userdata_iterator_with_fields,
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
            let settings = lua.to_value(&this.lockdown_set.settings()).map_err(|e| {
                LuaError::external(format!("Error while serializing settings: {}", e))
            })?;

            Ok(settings)
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(LuaMetaMethod::Len, |_, this, _: ()| {
            let len = this
                .lockdown_set
                .lockdowns()
                .len();

            Ok(len)
        });

        // Sorts the lockdowns by specificity in descending order
        methods.add_method_mut("sort", |_, this, _: ()| {
            this.lockdown_set.sort();

            Ok(())
        });

        methods.add_function(
            "apply",
            |_, (this, lockdown_type, reason): (LuaAnyUserData, LuaUserDataRef<LockdownMode>, String)| {
                Ok(
                    lua_promise!(this, lockdown_type, reason, |_lua, this, lockdown_type, reason|, {
                        let mut this = this
                            .borrow_mut::<LockdownSet<T>>()
                            .map_err(|_| LuaError::external("Failed to lock access to lockdown set. Please note that you cannot apply/remove multiple lockdowns at the same time"))?;
                        
                        this.check_action(lockdown_type.0.string_form())?;

                        match this
                            .lockdown_set
                            .apply(lockdown_type.0, &reason)
                            .await {
                            Ok(lockdown_id) => Ok(LockdownAddStatus::Ok(lockdown_id)),
                            Err(e) => match e {
                                lockdowns::LockdownError::LockdownTestFailed(e) => {
                                    Ok(LockdownAddStatus::LockdownTestFailed(LockdownTestResult(
                                        Rc::new(e),
                                    )))
                                }
                                lockdowns::LockdownError::Error(e) => {
                                    Ok(LockdownAddStatus::Error(e.to_string()))
                                },
                            }
                        }
                    }),
                )
            },
        );

        methods.add_function(
            "remove",
            |_, (this, id,): (LuaAnyUserData, String,)| {
                Ok(
                    lua_promise!(this, id, |_lua, this, id|, {
                        let mut this = this
                            .borrow_mut::<LockdownSet<T>>()
                            .map_err(|_| LuaError::external("Failed to lock access to lockdown set. Please note that you cannot apply/remove multiple lockdowns at the same time"))?;

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
                                    )))
                                }
                                lockdowns::LockdownError::Error(e) => {
                                    Ok(LockdownRemoveStatus::Error(e.to_string()))
                                },
                            }
                        }
                    }),
                )
            },
        );
    }
}

pub enum LockdownAddStatus {
    Ok(uuid::Uuid),
    Error(String),
    LockdownTestFailed(LockdownTestResult),
}

impl LuaUserData for LockdownAddStatus {
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
                    LockdownAddStatus::Ok(_) => lua.to_value("Ok")?,
                    LockdownAddStatus::Error(_) => lua.to_value("Error")?,
                    LockdownAddStatus::LockdownTestFailed(_) => {
                        lua.to_value("LockdownTestFailed")?
                    }
                },
                "id" => match this {
                    LockdownAddStatus::Ok(id) => lua.to_value(&id.to_string())?,
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
                _ => mlua::Value::Nil,
            };

            Ok(v)
        });

        // Iter
        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<LockdownAddStatus>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "ok", "type", "id",
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

pub enum LockdownRemoveStatus {
    Ok,
    Error(String),
    LockdownTestFailed(LockdownTestResult),
}

impl LuaUserData for LockdownRemoveStatus {
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
                    LockdownRemoveStatus::Ok => lua.to_value("Ok")?,
                    LockdownRemoveStatus::Error(_) => lua.to_value("Error")?,
                    LockdownRemoveStatus::LockdownTestFailed(_) => {
                        lua.to_value("LockdownTestFailed")?
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
                _ => mlua::Value::Nil,
            };

            Ok(v)
        });

        // Iter
        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<LockdownRemoveStatus>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "ok", "type",
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
pub struct LockdownTestResult(Rc<lockdowns::LockdownTestResult>);

impl LuaUserData for LockdownTestResult {
    // TODO
}
