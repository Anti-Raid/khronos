use ar_settings::types::{Setting, SettingCreator, SettingDeleter, SettingView, SettingsError};
use ar_settings::value::Value;
use serenity::async_trait;

pub struct PageProviderPage<InnerData>
where
    InnerData: Clone + Send + Sync,
{
    pub title: String,
    pub description: String,
    pub settings: Vec<Setting<InnerData>>,
}

/// A dummy settings op that does nothing
#[derive(Clone, Default)]
pub struct DummySettingOp<InnerData: Clone + Send + Sync> {
    pub _marker: std::marker::PhantomData<InnerData>,
}

#[async_trait]
impl<InnerData> SettingView<InnerData> for DummySettingOp<InnerData>
where
    InnerData: Clone + Send + Sync,
{
    async fn view<'a>(
        &self,
        _context: &InnerData,
        _filters: indexmap::IndexMap<String, Value>,
    ) -> Result<Vec<indexmap::IndexMap<String, Value>>, SettingsError> {
        Err(SettingsError::Generic {
            message: "Unsupported".to_string(),
            src: "khronos".to_string(),
            typ: "internal".to_string(),
        })
    }
}

#[async_trait]
impl<InnerData> SettingCreator<InnerData> for DummySettingOp<InnerData>
where
    InnerData: Clone + Send + Sync,
{
    async fn create<'a>(
        &self,
        _context: &InnerData,
        _data: indexmap::IndexMap<String, Value>,
    ) -> Result<indexmap::IndexMap<String, Value>, SettingsError> {
        Err(SettingsError::Generic {
            message: "Unsupported".to_string(),
            src: "khronos".to_string(),
            typ: "internal".to_string(),
        })
    }
}

#[async_trait]
impl<InnerData> SettingDeleter<InnerData> for DummySettingOp<InnerData>
where
    InnerData: Clone + Send + Sync,
{
    async fn delete<'a>(&self, _context: &InnerData, _key: Value) -> Result<(), SettingsError> {
        Err(SettingsError::Generic {
            message: "Unsupported".to_string(),
            src: "khronos".to_string(),
            typ: "internal".to_string(),
        })
    }
}

/// A page provider
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait PageProvider<InnerData>: 'static + Clone
where
    InnerData: Clone + Send + Sync,
{
    /// Attempts an action on the bucket, incrementing/adjusting ratelimits if needed
    ///
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str) -> Result<(), crate::Error>;

    /// Gets the current page for a template
    async fn get_page(&self) -> Result<PageProviderPage<InnerData>, crate::Error>;

    /// Sets the current page for a template
    ///
    /// Note that this method must also set settingsoperation as desired. By default, a dummy
    /// implementation is provided that does nothing for serde purposes
    async fn set_page(&self, page: PageProviderPage<InnerData>) -> Result<(), crate::Error>;

    /// Deletes the current page for a template
    async fn delete_page(&self) -> Result<(), crate::Error>;
}
