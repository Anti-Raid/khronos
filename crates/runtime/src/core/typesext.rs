use crate::primitives::create_userdata_iterator_with_fields;
use mlua::prelude::*;
use rand::distributions::{Alphanumeric, DistString};

/// Syntactically:
///
/// Null: `None`
/// Empty object: `Some(None)`
/// Value: `Some(Some(value))`
pub struct MultiOption<T: for<'a> serde::Deserialize<'a> + serde::Serialize> {
    inner: Option<Option<T>>,
}

impl<T: for<'a> serde::Deserialize<'a> + serde::Serialize + Clone> Clone for MultiOption<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: for<'a> serde::Deserialize<'a> + serde::Serialize + std::fmt::Debug> std::fmt::Debug
    for MultiOption<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MultiOption").field(&self.inner).finish()
    }
}

impl<T: for<'a> serde::Deserialize<'a> + serde::Serialize> Default for MultiOption<T> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl<T: for<'a> serde::Deserialize<'a> + serde::Serialize> MultiOption<T> {
    pub fn new(value: Option<T>) -> Self {
        Self {
            inner: value.map(Some),
        }
    }

    /// Returns true if the value is None
    pub fn is_none(&self) -> bool {
        self.inner.is_none()
    }

    /// Returns true if the value is Some(None)
    pub fn is_some(&self) -> bool {
        self.inner.is_some()
    }

    /// Returns true if the value is Some(Some(_))
    pub fn is_deep_some(&self) -> bool {
        match &self.inner {
            Some(Some(_)) => true,
            _ => false,
        }
    }

    pub fn as_inner_ref(&self) -> Option<&T> {
        self.inner.as_ref().and_then(Option::as_ref)
    }

    /// Returns true if the value should not be serialized
    ///
    /// E.g, the inner itself is None
    pub fn should_not_serialize(&self) -> bool {
        self.inner.is_none()
    }
}

// Deserialize
//
// If value is nil, we set it to None, if value is an empty object, we set it to Some(None), otherwise we set it to Some(Some(value))
impl<'de, T: for<'a> serde::Deserialize<'a> + serde::Serialize> serde::Deserialize<'de>
    for MultiOption<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Option<serde_json::Value> = Option::deserialize(deserializer)?;
        let inner = match value {
            None => None,
            Some(v) if v.is_object() && v.as_object().unwrap().is_empty() => Some(None),
            Some(v) => Some(Some(
                serde_json::from_value(v).map_err(serde::de::Error::custom)?,
            )),
        };
        Ok(Self { inner })
    }
}

// Serialize impl
impl<T: for<'a> serde::Deserialize<'a> + serde::Serialize> serde::Serialize for MultiOption<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;
        match &self.inner {
            None => Err(S::Error::custom("internal error: serde skip_serializing_if should been set to MultiOption::should_not_serialize")),
            Some(None) => serializer.serialize_none(), // We want to send null in this case
            Some(Some(value)) => value.serialize(serializer),
        }
    }
}

impl<T: for<'a> serde::Deserialize<'a> + serde::Serialize> std::ops::Deref for MultiOption<T> {
    type Target = Option<Option<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// U64 type
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U64(pub u64);

impl FromLua for U64 {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Integer(i) if i >= 0 => Ok(U64(i as u64)),
            LuaValue::String(s) => {
                let str_value = s.to_str()?;
                str_value
                    .parse::<u64>()
                    .map(U64)
                    .map_err(|_| LuaError::FromLuaConversionError {
                        from: "string",
                        to: "U64".to_string(),
                        message: Some("Value must be a non-negative integer".to_string()),
                    })
            }
            LuaValue::UserData(u) => {
                let u64 = u
                    .borrow::<U64>()
                    .map_err(|_| LuaError::FromLuaConversionError {
                        from: "UserData",
                        to: "U64".to_string(),
                        message: Some("UserData must be a U64".to_string()),
                    })?;

                Ok(U64(u64.0))
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: "{integer | string}",
                to: "U64".to_string(),
                message: Some("Value must be a non-negative integer or a string".to_string()),
            }),
        }
    }
}

impl LuaUserData for U64 {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Metamethods
        methods.add_meta_method(LuaMetaMethod::Add, |_, this, value: U64| {
            let v = this.0.wrapping_add(value.0);
            Ok(U64(v))
        });

        methods.add_meta_method(LuaMetaMethod::Sub, |_, this, value: U64| {
            let v = this.0.wrapping_sub(value.0);
            Ok(U64(v))
        });

        methods.add_meta_method(LuaMetaMethod::Mul, |_, this, value: U64| {
            let v = this.0.wrapping_mul(value.0);
            Ok(U64(v))
        });

        methods.add_meta_method(LuaMetaMethod::Div, |_, this, value: U64| {
            let v = this.0.wrapping_div(value.0);
            Ok(U64(v))
        });

        methods.add_meta_method(LuaMetaMethod::Mod, |_, this, value: U64| {
            let v = this.0.wrapping_rem(value.0);
            Ok(U64(v))
        });

        methods.add_meta_method(LuaMetaMethod::Pow, |_, this, value: u32| {
            let v = this.0.wrapping_pow(value);
            Ok(U64(v))
        });

        methods.add_meta_method(LuaMetaMethod::IDiv, |_, this, value: U64| {
            // Same as Div
            let v = this.0.wrapping_div(value.0);
            Ok(U64(v))
        });

        // Comparison
        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, value: U64| {
            Ok(this.0 == value.0)
        });

        methods.add_meta_method(
            LuaMetaMethod::Lt,
            |_, this, value: U64| Ok(this.0 < value.0),
        );

        methods.add_meta_method(LuaMetaMethod::Le, |_, this, value: U64| {
            Ok(this.0 <= value.0)
        });

        // Returns the string representation of the U64 value
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            Ok(this.0.to_string())
        });

        // Type
        methods.add_meta_method(LuaMetaMethod::Type, |_, _, _: ()| Ok("U64".to_string()));

        // Byte related methods
        methods.add_method("to_ne_bytes", |_, this, _: ()| {
            let bytes = this.0.to_ne_bytes();
            Ok(bytes.to_vec())
        });

        methods.add_function("from_ne_bytes", |_, bytes: Vec<u8>| {
            if bytes.len() != 8 {
                return Err(LuaError::external("Byte array must be of length 8"));
            }
            let array: [u8; 8] = bytes
                .try_into()
                .map_err(|_| LuaError::external("Failed to convert Vec<u8> to [u8; 8]"))?;
            let value = u64::from_ne_bytes(array);
            Ok(U64(value))
        });

        methods.add_method("to_le_bytes", |_, this, _: ()| {
            let bytes = this.0.to_le_bytes();
            Ok(bytes.to_vec())
        });

        methods.add_function("from_le_bytes", |_, bytes: Vec<u8>| {
            if bytes.len() != 8 {
                return Err(LuaError::external("Byte array must be of length 8"));
            }
            let array: [u8; 8] = bytes
                .try_into()
                .map_err(|_| LuaError::external("Failed to convert Vec<u8> to [u8; 8]"))?;
            let value = u64::from_le_bytes(array);
            Ok(U64(value))
        });

        methods.add_method("to_be_bytes", |_, this, _: ()| {
            let bytes = this.0.to_be_bytes();
            Ok(bytes.to_vec())
        });

        methods.add_function("from_be_bytes", |_, bytes: Vec<u8>| {
            if bytes.len() != 8 {
                return Err(LuaError::external("Byte array must be of length 8"));
            }
            let array: [u8; 8] = bytes
                .try_into()
                .map_err(|_| LuaError::external("Failed to convert Vec<u8> to [u8; 8]"))?;
            let value = u64::from_be_bytes(array);
            Ok(U64(value))
        });

        // Conversion methods
        methods.add_method("to_i64", |_, this, _: ()| {
            if this.0 > i64::MAX as u64 {
                return Err(LuaError::external("Value is too large to convert to i64"));
            }

            Ok(I64(this.0 as i64))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<U64>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Methods
                    "to_ne_bytes",
                    "from_ne_bytes",
                    "to_le_bytes",
                    "from_le_bytes",
                    "to_be_bytes",
                    "from_be_bytes",
                    "to_i64",
                ],
            )
        });
    }
}

// U64 type
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I64(pub i64);

impl FromLua for I64 {
    fn from_lua(value: LuaValue, _lua: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Integer(i) => Ok(I64(i)),
            LuaValue::String(s) => {
                let str_value = s.to_str()?;
                str_value
                    .parse::<i64>()
                    .map(I64)
                    .map_err(|_| LuaError::FromLuaConversionError {
                        from: "string",
                        to: "I64".to_string(),
                        message: Some("Value must be an integer".to_string()),
                    })
            }
            LuaValue::UserData(u) => {
                if u.is::<U64>() {
                    // Try converting the U64 to I64
                    let u64t = u
                        .borrow::<U64>()
                        .map_err(|_| LuaError::FromLuaConversionError {
                            from: "UserData",
                            to: "I64".to_string(),
                            message: Some("UserData must be a U64".to_string()),
                        })?;

                    if u64t.0 > i64::MAX as u64 {
                        return Err(LuaError::FromLuaConversionError {
                            from: "UserData",
                            to: "I64".to_string(),
                            message: Some("Value is too large to convert to i64".to_string()),
                        });
                    }

                    return Ok(I64(u64t.0 as i64));
                }

                let i64 = u
                    .borrow::<I64>()
                    .map_err(|_| LuaError::FromLuaConversionError {
                        from: "UserData",
                        to: "U64".to_string(),
                        message: Some("UserData must be a I64".to_string()),
                    })?;

                Ok(I64(i64.0))
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: "{integer | string}",
                to: "I64".to_string(),
                message: Some("Value must be an integer or a string".to_string()),
            }),
        }
    }
}

impl LuaUserData for I64 {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Metamethods
        methods.add_meta_method(LuaMetaMethod::Add, |_, this, value: I64| {
            let v = this.0.wrapping_add(value.0);
            Ok(I64(v))
        });

        methods.add_meta_method(LuaMetaMethod::Sub, |_, this, value: I64| {
            let v = this.0.wrapping_sub(value.0);
            Ok(I64(v))
        });

        methods.add_meta_method(LuaMetaMethod::Mul, |_, this, value: I64| {
            let v = this.0.wrapping_mul(value.0);
            Ok(I64(v))
        });

        methods.add_meta_method(LuaMetaMethod::Div, |_, this, value: I64| {
            let v = this.0.wrapping_div(value.0);
            Ok(I64(v))
        });

        methods.add_meta_method(LuaMetaMethod::Mod, |_, this, value: I64| {
            let v = this.0.wrapping_rem(value.0);
            Ok(I64(v))
        });

        methods.add_meta_method(LuaMetaMethod::Pow, |_, this, value: u32| {
            let v = this.0.wrapping_pow(value);
            Ok(I64(v))
        });

        methods.add_meta_method(LuaMetaMethod::IDiv, |_, this, value: I64| {
            // Same as Div
            let v = this.0.wrapping_div(value.0);
            Ok(I64(v))
        });

        // Comparison
        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, value: I64| {
            Ok(this.0 == value.0)
        });

        methods.add_meta_method(
            LuaMetaMethod::Lt,
            |_, this, value: I64| Ok(this.0 < value.0),
        );

        methods.add_meta_method(LuaMetaMethod::Le, |_, this, value: I64| {
            Ok(this.0 <= value.0)
        });

        // Returns the string representation of the U64 value
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            Ok(this.0.to_string())
        });

        // Type
        methods.add_meta_method(LuaMetaMethod::Type, |_, _, _: ()| Ok("I64".to_string()));

        // Byte related methods
        methods.add_method("to_ne_bytes", |_, this, _: ()| {
            let bytes = this.0.to_ne_bytes();
            Ok(bytes.to_vec())
        });

        methods.add_function("from_ne_bytes", |_, bytes: Vec<u8>| {
            if bytes.len() != 8 {
                return Err(LuaError::external("Byte array must be of length 8"));
            }
            let array: [u8; 8] = bytes
                .try_into()
                .map_err(|_| LuaError::external("Failed to convert Vec<u8> to [u8; 8]"))?;
            let value = u64::from_ne_bytes(array);
            Ok(U64(value))
        });

        methods.add_method("to_le_bytes", |_, this, _: ()| {
            let bytes = this.0.to_le_bytes();
            Ok(bytes.to_vec())
        });

        methods.add_function("from_le_bytes", |_, bytes: Vec<u8>| {
            if bytes.len() != 8 {
                return Err(LuaError::external("Byte array must be of length 8"));
            }
            let array: [u8; 8] = bytes
                .try_into()
                .map_err(|_| LuaError::external("Failed to convert Vec<u8> to [u8; 8]"))?;
            let value = u64::from_le_bytes(array);
            Ok(U64(value))
        });

        methods.add_method("to_be_bytes", |_, this, _: ()| {
            let bytes = this.0.to_be_bytes();
            Ok(bytes.to_vec())
        });

        methods.add_function("from_be_bytes", |_, bytes: Vec<u8>| {
            if bytes.len() != 8 {
                return Err(LuaError::external("Byte array must be of length 8"));
            }
            let array: [u8; 8] = bytes
                .try_into()
                .map_err(|_| LuaError::external("Failed to convert Vec<u8> to [u8; 8]"))?;
            let value = u64::from_be_bytes(array);
            Ok(U64(value))
        });

        // Conversion methods
        methods.add_method("to_u64", |_, this, _: ()| {
            if this.0 < 0 {
                return Err(LuaError::external(
                    "Value is negative, cannot convert to u64",
                ));
            }

            Ok(U64(this.0 as u64))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<I64>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Methods
                    "to_ne_bytes",
                    "from_ne_bytes",
                    "to_le_bytes",
                    "from_le_bytes",
                    "to_be_bytes",
                    "from_be_bytes",
                    "to_u64",
                ],
            )
        });
    }
}

/// Aim is to implement as many functions from https://luau.org/library#bit32-library as possible but for 64 bit unsigned integers
///
/// arshift, band, bnot, bor, bxor, btest, extract, lrotate, lshift, replace, rrotate, rshift, countlz
/// countrz, byteswap
///
/// Implemented:
/// - band
/// - bnot
/// - bor
/// - bxor
/// - btest
/// - extract
/// - lrotate
/// - lshift
/// - replace
/// - rrotate
/// - rshift
/// - countlz
/// - countrz
/// - byteswap
///
/// Not yet implemented due to spec difficulties:
/// - arshift
fn bitu64(lua: &Lua) -> LuaResult<LuaTable> {
    let submodule = lua.create_table()?;

    submodule.set(
        "band",
        lua.create_function(|lua, values: LuaMultiValue| {
            if values.is_empty() {
                // Return all 1s
                return Ok(U64(u64::MAX));
            }

            let mut result = u64::MAX;

            for value in values {
                let u64_value = U64::from_lua(value, lua)?;
                result &= u64_value.0;
            }

            Ok(U64(result))
        })?,
    )?;

    submodule.set(
        "bnot",
        lua.create_function(|_lua, n: U64| {
            let result = !n.0;
            Ok(U64(result))
        })?,
    )?;

    submodule.set(
        "bor",
        lua.create_function(|lua, values: LuaMultiValue| {
            if values.is_empty() {
                // Return 0
                return Ok(U64(0));
            }

            let mut result = u64::MAX;

            for value in values {
                let u64_value = U64::from_lua(value, lua)?;
                result |= u64_value.0;
            }

            Ok(U64(result))
        })?,
    )?;

    submodule.set(
        "bxor",
        lua.create_function(|lua, values: LuaMultiValue| {
            if values.is_empty() {
                // Return 0
                return Ok(U64(0));
            }

            let mut result = u64::MAX;

            for value in values {
                let u64_value = U64::from_lua(value, lua)?;
                result ^= u64_value.0;
            }

            Ok(U64(result))
        })?,
    )?;

    submodule.set(
        "btest",
        lua.create_function(|lua, values: LuaMultiValue| {
            if values.is_empty() {
                // band != 0
                return Ok(true);
            }

            let mut result = u64::MAX;

            for value in values {
                let u64_value = U64::from_lua(value, lua)?;
                result &= u64_value.0;
            }

            Ok(result != 0)
        })?,
    )?;

    submodule.set(
        "extract",
        lua.create_function(|_lua, (n, f, w): (U64, u32, Option<u32>)| {
            let w = w.unwrap_or(1);

            if f > 63 {
                return Err(LuaError::external("F must be less than 64"));
            }

            let m = f
                .checked_add(w)
                .ok_or(LuaError::external("F + W - 1 must be less than 64"))?
                .checked_sub(1)
                .ok_or(LuaError::external("F + W - 1 must be less than 64"))?;

            if m > 63 {
                return Err(LuaError::external("F + W must be less than 64"));
            }

            // Extract W bits at from N at position F
            // UNTESTED
            let mask = (1u64 << w) - 1;
            let result = (n.0 >> f) & mask;

            Ok(U64(result))
        })?,
    )?;

    submodule.set(
        "lrotate",
        lua.create_function(|_lua, (n, i): (U64, i64)| {
            if i < 0 {
                // Right rotate
                let result = n.0.rotate_right(
                    (-i).try_into()
                        .map_err(|_| LuaError::external("Invalid rotation value"))?,
                );
                Ok(U64(result))
            } else {
                let result = n.0.rotate_left(
                    i.try_into()
                        .map_err(|_| LuaError::external("Invalid rotation value"))?,
                );
                Ok(U64(result))
            }
        })?,
    )?;

    submodule.set(
        "lshift",
        lua.create_function(|_lua, (n, i): (U64, i64)| {
            if i < 0 {
                // Right shift
                let result = n.0.wrapping_shr(
                    (-i).try_into()
                        .map_err(|_| LuaError::external("Invalid shift value"))?,
                );
                Ok(U64(result))
            } else {
                // Left shift
                let result = n.0.wrapping_shl(
                    i.try_into()
                        .map_err(|_| LuaError::external("Invalid shift value"))?,
                );
                Ok(U64(result))
            }
        })?,
    )?;

    submodule.set(
        "replace",
        lua.create_function(|_lua, (n, r, f, w): (U64, u8, u32, Option<u32>)| {
            if r != 0 && r != 1 {
                return Err(LuaError::external("R must be 0 or 1"));
            }

            let w = w.unwrap_or(1);

            if f > 63 {
                return Err(LuaError::external("F must be less than 64"));
            }

            let m = f
                .checked_add(w)
                .ok_or(LuaError::external("F + W - 1 must be less than 64"))?
                .checked_sub(1)
                .ok_or(LuaError::external("F + W - 1 must be less than 64"))?;

            if m > 63 {
                return Err(LuaError::external("F + W must be less than 64"));
            }

            // Replace W bits at from N at position F
            // UNTESTED
            let mask = (1u64 << w) - 1;
            let result = (n.0 & !(mask << f)) | ((r as u64 & mask) << f);
            Ok(U64(result))
        })?,
    )?;

    submodule.set(
        "rrotate",
        lua.create_function(|_lua, (n, i): (U64, i64)| {
            if i < 0 {
                // Left rotate
                let result = n.0.rotate_left(
                    (-i).try_into()
                        .map_err(|_| LuaError::external("Invalid rotation value"))?,
                );
                Ok(U64(result))
            } else {
                let result = n.0.rotate_right(
                    i.try_into()
                        .map_err(|_| LuaError::external("Invalid rotation value"))?,
                );
                Ok(U64(result))
            }
        })?,
    )?;

    submodule.set(
        "rshift",
        lua.create_function(|_lua, (n, i): (U64, i64)| {
            if i < 0 {
                // Left shift
                let result = n.0.wrapping_shl(
                    (-i).try_into()
                        .map_err(|_| LuaError::external("Invalid shift value"))?,
                );
                Ok(U64(result))
            } else {
                // Left shift
                let result = n.0.wrapping_shr(
                    i.try_into()
                        .map_err(|_| LuaError::external("Invalid shift value"))?,
                );
                Ok(U64(result))
            }
        })?,
    )?;

    submodule.set(
        "countlz",
        lua.create_function(|_lua, n: U64| Ok(n.0.leading_zeros()))?,
    )?;

    submodule.set(
        "countrz",
        lua.create_function(|_lua, n: U64| Ok(n.0.trailing_zeros()))?,
    )?;

    submodule.set(
        "byteswap",
        lua.create_function(|_lua, n: U64| Ok(U64(n.0.swap_bytes())))?,
    )?;

    submodule.set_readonly(true); // Block any attempt to modify this table

    Ok(submodule)
}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "U64",
        lua.create_function(|lua, initial_value: LuaValue| {
            match initial_value {
                LuaValue::Nil => Ok(U64(0)), // Default value
                _ => {
                    let u64_value = U64::from_lua(initial_value, lua)?;
                    Ok(u64_value)
                }
            }
        })?,
    )?;

    module.set(
        "I64",
        lua.create_function(|lua, initial_value: LuaValue| {
            match initial_value {
                LuaValue::Nil => Ok(I64(0)), // Default value
                _ => {
                    let i64_value = I64::from_lua(initial_value, lua)?;
                    Ok(i64_value)
                }
            }
        })?,
    )?;

    module.set("bitu64", bitu64(lua)?)?;

    module.set(
        "randstring",
        lua.create_function(|_lua, length: usize| {
            if length == 0 || length > 255 {
                return Err(LuaError::external(
                    "Length must be greater than 0 and less than 256",
                ));
            }

            Ok(Alphanumeric.sample_string(&mut rand::thread_rng(), length))
        })?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}

#[cfg(test)]
mod tests {
    use mlua::prelude::*;

    #[test]
    fn test_multi_option() {
        let lua = mlua::Lua::new();

        lua.globals()
            .set(
                "testmo",
                lua.create_function(|lua, data: LuaValue| {
                    let v = lua.from_value::<super::MultiOption<u64>>(data)?;

                    #[derive(serde::Serialize)]
                    pub struct Dummy {
                        #[serde(skip_serializing_if = "super::MultiOption::should_not_serialize")]
                        a: super::MultiOption<u64>,
                    }

                    let d = Dummy { a: v.clone() };

                    println!("{:?}, serde: {:?}", v, serde_json::to_string(&d).unwrap());

                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.load(
            r#"
            print("Solid input")
            testmo(1)
            print("Empty object")
            testmo({})
            print("Actual unpassed")
            testmo(nil)
        "#,
        )
        .exec()
        .unwrap();
    }
}
