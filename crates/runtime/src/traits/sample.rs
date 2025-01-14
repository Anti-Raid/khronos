use antiraid_types::userinfo::UserInfo;

use crate::utils::executorscope::ExecutorScope;

use super::context::KhronosContext;
use super::discordprovider::DiscordProvider;
use super::kvprovider::{self, KVProvider};
use super::lockdownprovider::LockdownProvider;
use super::userinfoprovider::UserInfoProvider;

#[derive(Clone)]
pub struct SampleKhronosContext {
    v: Vec<String>,
}

impl KhronosContext for SampleKhronosContext {
    type Data = ();
    type KVProvider = SampleKVProvider;
    type DiscordProvider = SampleDiscordProvider;
    type LockdownProvider = SampleLockdownProvider;
    type UserInfoProvider = SampleUserInfoProvider;

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

    fn kv_provider(&self, _scope: ExecutorScope) -> Option<Self::KVProvider> {
        Some(SampleKVProvider {})
    }

    fn discord_provider(&self, _scope: ExecutorScope) -> Option<Self::DiscordProvider> {
        Some(SampleDiscordProvider {})
    }

    fn lockdown_provider(&self, _scope: ExecutorScope) -> Option<Self::LockdownProvider> {
        Some(SampleLockdownProvider {})
    }

    fn userinfo_provider(&self, _scope: ExecutorScope) -> Option<Self::UserInfoProvider> {
        Some(SampleUserInfoProvider {})
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

    async fn get_automod_rules(
        &self,
    ) -> serenity::Result<Vec<serenity::model::guild::automod::Rule>> {
        todo!()
    }

    async fn get_automod_rule(
        &self,
        _rule_id: serenity::all::RuleId,
    ) -> serenity::Result<serenity::model::guild::automod::Rule> {
        todo!()
    }

    async fn edit_channel(
        &self,
        _channel_id: serenity::all::ChannelId,
        _map: impl serde::Serialize,
        _audit_log_reason: Option<&str>,
    ) -> serenity::Result<serenity::model::channel::Channel> {
        todo!()
    }

    async fn delete_channel(
        &self,
        _channel_id: serenity::all::ChannelId,
        _audit_log_reason: Option<&str>,
    ) -> serenity::Result<serenity::model::channel::Channel> {
        todo!()
    }

    async fn create_member_ban(
        &self,
        _user_id: serenity::all::UserId,
        _delete_message_seconds: u32,
        _reason: Option<&str>,
    ) -> serenity::Result<()> {
        todo!()
    }

    async fn kick_member(
        &self,
        _user_id: serenity::all::UserId,
        _reason: Option<&str>,
    ) -> serenity::Result<()> {
        todo!()
    }

    async fn edit_member(
        &self,
        _user_id: serenity::all::UserId,
        _map: impl serde::Serialize,
        _audit_log_reason: Option<&str>,
    ) -> serenity::Result<serenity::all::Member> {
        todo!()
    }

    async fn send_message(
        &self,
        _channel_id: serenity::all::ChannelId,
        _files: Vec<serenity::all::CreateAttachment<'_>>,
        _data: impl serde::Serialize,
    ) -> serenity::Result<serenity::model::channel::Message> {
        todo!()
    }

    async fn create_interaction_response(
        &self,
        _interaction_id: serenity::all::InteractionId,
        _interaction_token: &str,
        _response: impl serde::Serialize,
        _files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> serenity::Result<()> {
        todo!()
    }

    async fn get_original_interaction_response(
        &self,
        _interaction_token: &str,
    ) -> serenity::Result<serenity::model::channel::Message> {
        todo!()
    }

    async fn get_guild_commands(&self) -> serenity::Result<Vec<serenity::all::Command>> {
        todo!()
    }

    async fn get_guild_command(
        &self,
        _command_id: serenity::all::CommandId,
    ) -> serenity::Result<serenity::all::Command> {
        todo!()
    }

    async fn create_guild_command(
        &self,
        _map: impl serde::Serialize,
    ) -> serenity::Result<serenity::all::Command> {
        todo!()
    }
}

#[derive(Clone)]
pub struct SampleLockdownProvider {}

impl LockdownProvider for SampleLockdownProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), crate::Error> {
        todo!()
    }

    async fn list(&self) -> Result<Vec<super::lockdownprovider::Lockdown>, crate::Error> {
        todo!()
    }

    async fn qsl(&self, _reason: String) -> Result<uuid::Uuid, crate::Error> {
        todo!()
    }

    async fn tsl(&self, _reason: String) -> Result<uuid::Uuid, crate::Error> {
        todo!()
    }

    async fn scl(
        &self,
        _channel_id: serenity::all::ChannelId,
        _reason: String,
    ) -> Result<uuid::Uuid, crate::Error> {
        todo!()
    }

    async fn role(
        &self,
        _role_id: serenity::all::RoleId,
        _reason: String,
    ) -> Result<uuid::Uuid, crate::Error> {
        todo!()
    }

    async fn remove(&self, _id: uuid::Uuid) -> Result<(), crate::Error> {
        todo!()
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
