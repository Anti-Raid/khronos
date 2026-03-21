use crate::{TemplateContext, traits::runtimeprovider::RuntimeProvider};

use super::{
    httpclientprovider::HTTPClientProvider,
    kvprovider::KVProvider, objectstorageprovider::ObjectStorageProvider,
    globalkvprovider::GlobalKVProvider
};
use dapi::controller::DiscordProvider;
use mluau::prelude::*;

pub trait KhronosContext: 'static + Clone + Sized {
    type KVProvider: KVProvider;
    type GlobalKVProvider: GlobalKVProvider;
    type DiscordProvider: DiscordProvider;
    type ObjectStorageProvider: ObjectStorageProvider;
    type HTTPClientProvider: HTTPClientProvider;
    type RuntimeProvider: RuntimeProvider;

    /// Returns a key-value provider
    fn kv_provider(&self) -> Option<Self::KVProvider>;

    /// Returns a global key-value provider
    fn global_kv_provider(&self) -> Option<Self::GlobalKVProvider>;

    /// Returns a Discord provider
    ///
    /// This is used to interact with Discord API
    fn discord_provider(&self) -> Option<Self::DiscordProvider>;

    /// Returns a ObjectStorage provider
    fn objectstorage_provider(&self) -> Option<Self::ObjectStorageProvider>;

    /// Returns a HTTP client provider
    fn httpclient_provider(&self) -> Option<Self::HTTPClientProvider>;

    /// Returns a runtime provider
    fn runtime_provider(&self) -> Option<Self::RuntimeProvider>;

    /// Returns the contexts memory limit, if any
    fn memory_limit(&self) -> Option<usize> {
        None
    }

    /// Returns any additional plugins to bind to the context
    fn extra_plugins() -> indexmap::IndexMap<String, Box<dyn Fn(&Lua, &TemplateContext<Self>) -> LuaResult<LuaValue>>> {
        indexmap::IndexMap::new()
    }
}
