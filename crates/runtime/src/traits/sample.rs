use crate::utils::executorscope::ExecutorScope;

use super::context::KhronosContext;
use super::kvprovider::{self, KVProvider};

#[derive(Clone)]
pub struct SampleKhronosContext {
    v: Vec<String>,
}

impl KhronosContext for SampleKhronosContext {
    type Data = ();
    type KVProvider = SampleKVProvider;

    fn data(&self) -> Self::Data {
        todo!()
    }

    fn allowed_caps(&self) -> &[String] {
        self.v.as_ref()
    }

    fn guild_id(&self) -> Option<serenity::all::GuildId> {
        None
    }

    fn owner_guild_id(&self) -> Option<serenity::all::GuildId> {
        None
    }

    fn current_user(&self) -> Option<serenity::all::CurrentUser> {
        None::<serenity::all::CurrentUser>
    }

    fn kv_executor(&self, _scope: ExecutorScope) -> Option<Self::KVProvider> {
        None
    }
}

#[derive(Clone)]
pub struct SampleKVProvider {}

impl KVProvider for SampleKVProvider {
    async fn get(&self, _key: String) -> Result<Option<kvprovider::KvRecord>, crate::Error> {
        todo!()
    }

    async fn set(&self, _key: String, _value: serde_json::Value) -> Result<(), crate::Error> {
        todo!()
    }

    async fn delete(&self, _key: String) -> Result<(), crate::Error> {
        todo!()
    }

    fn attempt_action(&self, _bucket: &str) -> Result<(), crate::Error> {
        todo!()
    }

    async fn find(&self, _query: String) -> Result<Vec<super::kvprovider::KvRecord>, crate::Error> {
        todo!()
    }

    async fn exists(&self, _key: String) -> Result<bool, crate::Error> {
        todo!()
    }

    async fn keys(&self) -> Result<Vec<String>, crate::Error> {
        todo!()
    }
}
