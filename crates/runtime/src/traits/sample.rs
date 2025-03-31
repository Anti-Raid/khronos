use std::rc::Rc;

use antiraid_types::userinfo::UserInfo;

use crate::utils::executorscope::ExecutorScope;

use super::context::KhronosContext;
use super::discordprovider::DiscordProvider;
use super::kvprovider::KVProvider;
use super::lockdownprovider::LockdownProvider;
use super::pageprovider::PageProvider;
use super::stingprovider::StingProvider;
use super::userinfoprovider::UserInfoProvider;
use antiraid_types::stings::{Sting, StingAggregate, StingCreate};

#[derive(Clone, Default)]
pub struct SampleKhronosContext {
    v: Vec<String>,
    _d: Option<serenity::all::Context>,
    _rc: Rc<String>,
}

#[derive(Clone)]
pub struct SampleLockdownDataStore {}

impl lockdowns::LockdownDataStore for SampleLockdownDataStore {
    async fn get_guild_lockdown_settings(
        &self,
        _guild_id: serenity::all::GuildId,
    ) -> Result<lockdowns::GuildLockdownSettings, lockdowns::Error> {
        todo!()
    }

    async fn get_lockdowns(
        &self,
        _guild_id: serenity::all::GuildId,
    ) -> Result<Vec<lockdowns::Lockdown>, lockdowns::Error> {
        todo!()
    }

    async fn insert_lockdown(
        &self,
        _guild_id: serenity::all::GuildId,
        _lockdown: lockdowns::CreateLockdown,
    ) -> Result<lockdowns::Lockdown, lockdowns::Error> {
        todo!()
    }

    async fn remove_lockdown(
        &self,
        _guild_id: serenity::all::GuildId,
        _id: uuid::Uuid,
    ) -> Result<(), lockdowns::Error> {
        todo!()
    }

    async fn guild(
        &self,
        _guild_id: serenity::all::GuildId,
    ) -> Result<serenity::all::PartialGuild, lockdowns::Error> {
        todo!()
    }

    async fn guild_channels(
        &self,
        _guild_id: serenity::all::GuildId,
    ) -> Result<Vec<serenity::all::GuildChannel>, lockdowns::Error> {
        todo!()
    }

    fn cache(&self) -> Option<&serenity::all::Cache> {
        todo!()
    }

    fn http(&self) -> &serenity::all::Http {
        todo!()
    }
}

impl KhronosContext for SampleKhronosContext {
    type Data = ();
    type KVProvider = SampleKVProvider;
    type DiscordProvider = SampleDiscordProvider;
    type LockdownDataStore = SampleLockdownDataStore;
    type LockdownProvider = SampleLockdownProvider;
    type UserInfoProvider = SampleUserInfoProvider;
    type StingProvider = SampleStingProvider;
    type PageProvider = DummyPageProvider;
    type AssetManager = crate::utils::assets::FileAssetManager;

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

    fn isolate(&self) -> &crate::rt::KhronosIsolate<Self::AssetManager> {
        todo!()
    }

    fn kv_provider(&self, _scope: ExecutorScope) -> Option<Self::KVProvider> {
        Some(SampleKVProvider {})
    }

    fn discord_provider(&self, _scope: ExecutorScope) -> Option<Self::DiscordProvider> {
        Some(SampleDiscordProvider {})
    }

    fn lockdown_provider(&self, _scope: ExecutorScope) -> Option<Self::LockdownProvider> {
        Some(SampleLockdownProvider {
            _store: SampleLockdownDataStore {},
        })
    }

    fn userinfo_provider(&self, _scope: ExecutorScope) -> Option<Self::UserInfoProvider> {
        Some(SampleUserInfoProvider {})
    }

    fn sting_provider(&self, _scope: ExecutorScope) -> Option<Self::StingProvider> {
        Some(SampleStingProvider {})
    }

    fn page_provider(&self, _scope: ExecutorScope) -> Option<Self::PageProvider> {
        Some(DummyPageProvider {})
    }
}

#[derive(Clone)]
pub struct SampleKVProvider {}

impl KVProvider for SampleKVProvider {
    async fn get(&self, _key: String) -> Result<Option<crate::traits::ir::KvRecord>, crate::Error> {
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

    async fn find(&self, _query: String) -> Result<Vec<crate::traits::ir::KvRecord>, crate::Error> {
        todo!()
    }

    async fn exists(&self, _key: String) -> Result<bool, crate::Error> {
        todo!()
    }

    async fn keys(&self) -> Result<Vec<String>, crate::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct SampleDiscordProvider {}

impl DiscordProvider for SampleDiscordProvider {
    fn attempt_action(&self, _bucket: &str) -> serenity::Result<(), crate::Error> {
        todo!()
    }

    async fn guild(
        &self,
    ) -> serenity::Result<serenity::model::prelude::PartialGuild, crate::Error> {
        todo!()
    }

    async fn member(
        &self,
        _user_id: serenity::all::UserId,
    ) -> serenity::Result<Option<serenity::all::Member>, crate::Error> {
        todo!()
    }

    async fn guild_channel(
        &self,
        _channel_id: serenity::all::ChannelId,
    ) -> serenity::Result<serenity::all::GuildChannel, crate::Error> {
        todo!()
    }

    async fn get_audit_logs(
        &self,
        _action_type: Option<serenity::all::audit_log::Action>,
        _user_id: Option<serenity::model::prelude::UserId>,
        _before: Option<serenity::model::prelude::AuditLogEntryId>,
        _limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> serenity::Result<serenity::model::prelude::AuditLogs, crate::Error> {
        todo!()
    }

    async fn edit_channel(
        &self,
        _channel_id: serenity::all::ChannelId,
        _map: impl serde::Serialize,
        _audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::GuildChannel, crate::Error> {
        todo!()
    }

    async fn delete_channel(
        &self,
        _channel_id: serenity::all::ChannelId,
        _audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::Channel, crate::Error> {
        todo!()
    }

    async fn create_member_ban(
        &self,
        _user_id: serenity::all::UserId,
        _delete_message_seconds: u32,
        _reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        todo!()
    }

    async fn add_guild_member_role(
        &self,
        _user_id: serenity::all::UserId,
        _role_id: serenity::all::RoleId,
        _audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        todo!()
    }

    async fn remove_guild_member_role(
        &self,
        _user_id: serenity::all::UserId,
        _role_id: serenity::all::RoleId,
        _audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        todo!()
    }

    async fn remove_guild_member(
        &self,
        _user_id: serenity::all::UserId,
        _audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        todo!()
    }

    async fn get_guild_bans(
        &self,
        _target: Option<serenity::all::UserPagination>,
        _limit: Option<serenity::nonmax::NonMaxU16>,
    ) -> Result<Vec<serenity::all::Ban>, crate::Error> {
        todo!()
    }

    async fn kick_member(
        &self,
        _user_id: serenity::all::UserId,
        _reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        todo!()
    }

    async fn edit_member(
        &self,
        _user_id: serenity::all::UserId,
        _map: impl serde::Serialize,
        _audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::Member, crate::Error> {
        todo!()
    }

    async fn create_message(
        &self,
        _channel_id: serenity::all::ChannelId,
        _files: Vec<serenity::all::CreateAttachment<'_>>,
        _data: impl serde::Serialize,
    ) -> Result<serenity::model::channel::Message, crate::Error> {
        todo!()
    }

    async fn create_interaction_response(
        &self,
        _interaction_id: serenity::all::InteractionId,
        _interaction_token: &str,
        _response: impl serde::Serialize,
        _files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<(), crate::Error> {
        todo!()
    }

    async fn create_followup_message(
        &self,
        _interaction_token: &str,
        _response: impl serde::Serialize,
        _files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<serenity::all::Message, crate::Error> {
        todo!()
    }

    async fn get_original_interaction_response(
        &self,
        _interaction_token: &str,
    ) -> Result<serenity::model::channel::Message, crate::Error> {
        todo!()
    }

    async fn get_guild_commands(&self) -> Result<Vec<serenity::all::Command>, crate::Error> {
        todo!()
    }

    async fn get_guild_command(
        &self,
        _command_id: serenity::all::CommandId,
    ) -> Result<serenity::all::Command, crate::Error> {
        todo!()
    }

    async fn create_guild_command(
        &self,
        _map: impl serde::Serialize,
    ) -> Result<serenity::all::Command, crate::Error> {
        todo!()
    }

    async fn get_guild_roles(
        &self,
    ) -> Result<extract_map::ExtractMap<serenity::all::RoleId, serenity::all::Role>, crate::Error>
    {
        todo!()
    }

    async fn get_messages(
        &self,
        _channel_id: serenity::all::ChannelId,
        _target: Option<serenity::all::MessagePagination>,
        _limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> Result<Vec<serenity::all::Message>, crate::Error> {
        todo!()
    }

    async fn get_message(
        &self,
        _channel_id: serenity::all::ChannelId,
        _message_id: serenity::all::MessageId,
    ) -> Result<serenity::all::Message, crate::Error> {
        todo!()
    }

    async fn list_auto_moderation_rules(
        &self,
    ) -> Result<Vec<serenity::model::guild::automod::Rule>, crate::Error> {
        todo!()
    }

    async fn get_auto_moderation_rule(
        &self,
        _rule_id: serenity::all::RuleId,
    ) -> Result<serenity::model::guild::automod::Rule, crate::Error> {
        todo!()
    }

    async fn create_auto_moderation_rule(
        &self,
        _map: impl serde::Serialize,
        _audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::guild::automod::Rule, crate::Error> {
        todo!()
    }

    async fn edit_auto_moderation_rule(
        &self,
        _rule_id: serenity::all::RuleId,
        _map: impl serde::Serialize,
        _audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::guild::automod::Rule, crate::Error> {
        todo!()
    }

    async fn delete_auto_moderation_rule(
        &self,
        _rule_id: serenity::all::RuleId,
        _audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        todo!()
    }

    async fn edit_channel_permissions(
        &self,
        _channel_id: serenity::all::ChannelId,
        _target_id: serenity::all::TargetId,
        _data: impl serde::Serialize,
        _audit_log_reason: Option<&str>,
    ) -> Result<(), crate::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct SampleLockdownProvider {
    _store: SampleLockdownDataStore,
}

impl LockdownProvider<SampleLockdownDataStore> for SampleLockdownProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), crate::Error> {
        todo!()
    }

    fn lockdown_data_store(&self) -> &SampleLockdownDataStore {
        &self._store
    }
}

#[derive(Clone)]
pub struct SampleUserInfoProvider {}

impl UserInfoProvider for SampleUserInfoProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), crate::Error> {
        todo!()
    }

    async fn get(&self, _user_id: serenity::all::UserId) -> Result<UserInfo, crate::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct SampleStingProvider {}

impl StingProvider for SampleStingProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), crate::Error> {
        todo!()
    }

    async fn list(&self, _page: usize) -> Result<Vec<Sting>, crate::Error> {
        todo!()
    }

    async fn get(&self, _id: uuid::Uuid) -> Result<Option<Sting>, crate::Error> {
        todo!()
    }

    async fn create(&self, _sting: StingCreate) -> Result<uuid::Uuid, crate::Error> {
        todo!()
    }

    async fn update(&self, _sting: Sting) -> Result<(), crate::Error> {
        todo!()
    }

    async fn delete(&self, _id: uuid::Uuid) -> Result<(), crate::Error> {
        todo!()
    }

    /// Returns a StingAggregate set for a user in the guild
    async fn guild_user_aggregate(
        &self,
        _target: serenity::all::UserId,
    ) -> Result<Vec<StingAggregate>, crate::Error> {
        todo!()
    }

    /// Returns a StingAggregate set for the guild
    async fn guild_aggregate(&self) -> Result<Vec<StingAggregate>, crate::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct DummyPageProvider {}

impl PageProvider for DummyPageProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), crate::Error> {
        todo!()
    }

    async fn get_page(&self) -> Option<super::ir::Page> {
        todo!()
    }

    async fn set_page(&self, _page: super::ir::Page) -> Result<(), crate::Error> {
        todo!()
    }

    async fn delete_page(&self) -> Result<(), crate::Error> {
        todo!()
    }
}
