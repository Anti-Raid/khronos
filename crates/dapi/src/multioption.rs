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
        matches!(self.inner, Some(Some(_)))
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