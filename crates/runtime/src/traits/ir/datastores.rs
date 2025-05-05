use crate::utils::khronos_value::KhronosValue;
use mlua::prelude::*;
use serenity::async_trait;
use std::rc::Rc;
use std::{future::Future, pin::Pin};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeString {
    inner_str: String,
}

impl SafeString {
    pub fn is_safe(s: &str) -> bool {
        s.chars().all(|c| c.is_alphanumeric() || c == '_')
    }

    pub fn new(s: String) -> Option<Self> {
        if !Self::is_safe(&s) {
            return None;
        }

        Some(Self { inner_str: s })
    }
}

impl FromLua for SafeString {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        let s = String::from_lua(value, lua)?;
        let Some(safe_string) = SafeString::new(s) else {
            return Err(LuaError::FromLuaConversionError {
                from: "any",
                to: "SafeString".to_string(),
                message: Some(
                    "SafeStrings can only contain alphanumeric characters or underscores"
                        .to_string(),
                ),
            });
        };
        Ok(safe_string)
    }
}

impl IntoLua for SafeString {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        String::into_lua(self.inner_str, lua)
    }
}

impl std::fmt::Display for SafeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner_str)
    }
}

impl std::ops::Deref for SafeString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner_str
    }
}

impl serde::Serialize for SafeString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Ensure the string only contains either alphanumeric characters or underscores
        if !SafeString::is_safe(&self.inner_str) {
            return Err(serde::ser::Error::custom(
                "SafeStrings can only contain alphanumeric characters or underscores",
            ));
        }

        serializer.serialize_str(&self.inner_str)
    }
}

impl<'de> serde::Deserialize<'de> for SafeString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        SafeString::new(s).ok_or(serde::de::Error::custom(
            "SafeStrings can only contain alphanumeric characters or underscores",
        ))
    }
}

#[derive(Clone)]
pub enum DataStoreMethod {
    Async(
        Rc<
            dyn Fn(
                Vec<KhronosValue>,
            )
                -> Pin<Box<dyn Future<Output = Result<KhronosValue, crate::Error>>>>,
        >,
    ),
    Sync(Rc<dyn Fn(Vec<KhronosValue>) -> Result<KhronosValue, crate::Error>>),
}

impl
    From<
        Rc<
            dyn Fn(
                Vec<KhronosValue>,
            )
                -> Pin<Box<dyn Future<Output = Result<KhronosValue, crate::Error>>>>,
        >,
    > for DataStoreMethod
{
    fn from(
        f: Rc<
            dyn Fn(
                Vec<KhronosValue>,
            )
                -> Pin<Box<dyn Future<Output = Result<KhronosValue, crate::Error>>>>,
        >,
    ) -> Self {
        DataStoreMethod::Async(f)
    }
}
impl From<Rc<dyn Fn(Vec<KhronosValue>) -> Result<KhronosValue, crate::Error>>> for DataStoreMethod {
    fn from(f: Rc<dyn Fn(Vec<KhronosValue>) -> Result<KhronosValue, crate::Error>>) -> Self {
        DataStoreMethod::Sync(f)
    }
}

#[async_trait(?Send)]
pub trait DataStoreImpl {
    fn name(&self) -> String;
    fn need_caps(&self, method: &str) -> bool;

    /// Returns a list of methods
    fn methods(&self) -> Vec<String>;

    /// Gets a method
    fn get_method(&self, key: String) -> Option<DataStoreMethod>;
}

impl DataStoreImpl for Rc<dyn DataStoreImpl> {
    fn name(&self) -> String {
        self.as_ref().name()
    }

    fn need_caps(&self, method: &str) -> bool {
        self.as_ref().need_caps(method)
    }

    fn methods(&self) -> Vec<String> {
        self.as_ref().methods()
    }

    fn get_method(&self, key: String) -> Option<DataStoreMethod> {
        self.as_ref().get_method(key)
    }
}

/// A data store to copy whatever is input to copy()
pub struct CopyDataStore;

#[async_trait(?Send)]
impl DataStoreImpl for CopyDataStore {
    fn name(&self) -> String {
        "CopyDataStore".to_string()
    }

    fn need_caps(&self, _method: &str) -> bool {
        false
    }

    fn methods(&self) -> Vec<String> {
        vec!["copy".to_string(), "copySync".to_string()]
    }

    fn get_method(&self, key: String) -> Option<DataStoreMethod> {
        if key == "copy" {
            Some(DataStoreMethod::Async(Rc::new(|v| {
                Box::pin(async {
                    let mut v = v;
                    if v.len() == 0 {
                        return Ok(KhronosValue::Null);
                    } else if v.len() == 1 {
                        let Some(v) = v.pop() else {
                            return Ok(KhronosValue::Null);
                        };
                        return Ok(v);
                    } else {
                        return Ok(KhronosValue::List(v));
                    }
                })
            })))
        } else if key == "copySync" {
            Some(DataStoreMethod::Sync(Rc::new(|v| {
                let mut v = v;
                if v.len() == 0 {
                    return Ok(KhronosValue::Null);
                } else if v.len() == 1 {
                    let Some(v) = v.pop() else {
                        return Ok(KhronosValue::Null);
                    };
                    return Ok(v);
                } else {
                    return Ok(KhronosValue::List(v));
                }
            })))
        } else {
            None
        }
    }
}
