use std::rc::Rc;

use antiraid_types::stings::{Sting, StingAggregate, StingCreate};
use antiraid_types::userinfo::UserInfo;
use khronos_runtime::traits::context::KhronosContext;
use khronos_runtime::traits::discordprovider::DiscordProvider;
use khronos_runtime::traits::kvprovider::KVProvider;
use khronos_runtime::traits::lockdownprovider::LockdownProvider;
use khronos_runtime::traits::pageprovider::PageProvider;
use khronos_runtime::traits::stingprovider::StingProvider;
use khronos_runtime::traits::userinfoprovider::UserInfoProvider;
use khronos_runtime::utils::executorscope::ExecutorScope;

#[derive(Clone)]
pub struct CliKhronosContext {
    pub allowed_caps: Vec<String>,
    pub guild_id: Option<serenity::all::GuildId>,
    pub owner_guild_id: Option<serenity::all::GuildId>,
    pub global_table: mlua::Table,
    pub http: Option<Rc<serenity::all::Http>>,
}

impl KhronosContext for CliKhronosContext {
    type Data = ();
    type KVProvider = CliKVProvider;
    type DiscordProvider = CliDiscordProvider;
    type LockdownProvider = CliLockdownProvider;
    type UserInfoProvider = CliUserInfoProvider;
    type StingProvider = CliStingProvider;
    type PageProvider = CliPageProvider;

    fn data(&self) -> Self::Data {
        todo!()
    }

    fn allowed_caps(&self) -> &[String] {
        self.allowed_caps.as_ref()
    }

    fn guild_id(&self) -> Option<serenity::all::GuildId> {
        self.guild_id
    }

    fn owner_guild_id(&self) -> Option<serenity::all::GuildId> {
        self.owner_guild_id
    }

    fn current_user(&self) -> Option<serenity::all::CurrentUser> {
        None::<serenity::all::CurrentUser>
    }

    /// Returns the global table to use
    fn global_table(&self) -> mlua::Table {
        self.global_table.clone()
    }

    fn kv_provider(&self, _scope: ExecutorScope) -> Option<Self::KVProvider> {
        Some(CliKVProvider {})
    }

    fn discord_provider(&self, _scope: ExecutorScope) -> Option<Self::DiscordProvider> {
        Some(CliDiscordProvider {})
    }

    fn lockdown_provider(&self, _scope: ExecutorScope) -> Option<Self::LockdownProvider> {
        Some(CliLockdownProvider {})
    }

    fn userinfo_provider(&self, _scope: ExecutorScope) -> Option<Self::UserInfoProvider> {
        Some(CliUserInfoProvider {})
    }

    fn sting_provider(&self, _scope: ExecutorScope) -> Option<Self::StingProvider> {
        Some(CliStingProvider {})
    }

    fn page_provider(&self, _scope: ExecutorScope) -> Option<Self::PageProvider> {
        Some(CliPageProvider {})
    }
}

#[derive(Clone)]
pub struct CliKVProvider {}

impl KVProvider for CliKVProvider {
    async fn get(
        &self,
        _key: String,
    ) -> Result<Option<khronos_runtime::traits::ir::KvRecord>, khronos_runtime::Error> {
        todo!()
    }

    async fn set(
        &self,
        _key: String,
        _value: serde_json::Value,
    ) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn delete(&self, _key: String) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn find(
        &self,
        _query: String,
    ) -> Result<Vec<khronos_runtime::traits::ir::KvRecord>, khronos_runtime::Error> {
        todo!()
    }

    async fn exists(&self, _key: String) -> Result<bool, khronos_runtime::Error> {
        todo!()
    }

    async fn keys(&self) -> Result<Vec<String>, khronos_runtime::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct CliDiscordProvider {}

impl DiscordProvider for CliDiscordProvider {
    fn attempt_action(&self, _bucket: &str) -> serenity::Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn guild(
        &self,
    ) -> serenity::Result<serenity::model::prelude::PartialGuild, khronos_runtime::Error> {
        todo!()
    }

    async fn member(
        &self,
        _user_id: serenity::all::UserId,
    ) -> serenity::Result<Option<serenity::all::Member>, khronos_runtime::Error> {
        todo!()
    }

    async fn guild_channel(
        &self,
        _channel_id: serenity::all::ChannelId,
    ) -> serenity::Result<serenity::all::GuildChannel, khronos_runtime::Error> {
        todo!()
    }

    async fn get_audit_logs(
        &self,
        _action_type: Option<serenity::all::audit_log::Action>,
        _user_id: Option<serenity::model::prelude::UserId>,
        _before: Option<serenity::model::prelude::AuditLogEntryId>,
        _limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> serenity::Result<serenity::model::prelude::AuditLogs, khronos_runtime::Error> {
        todo!()
    }

    async fn get_automod_rules(
        &self,
    ) -> Result<Vec<serenity::model::guild::automod::Rule>, khronos_runtime::Error> {
        todo!()
    }

    async fn get_automod_rule(
        &self,
        _rule_id: serenity::all::RuleId,
    ) -> Result<serenity::model::guild::automod::Rule, khronos_runtime::Error> {
        todo!()
    }

    async fn edit_channel(
        &self,
        _channel_id: serenity::all::ChannelId,
        _map: impl serde::Serialize,
        _audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::GuildChannel, khronos_runtime::Error> {
        todo!()
    }

    async fn delete_channel(
        &self,
        _channel_id: serenity::all::ChannelId,
        _audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::Channel, khronos_runtime::Error> {
        todo!()
    }

    async fn create_member_ban(
        &self,
        _user_id: serenity::all::UserId,
        _delete_message_seconds: u32,
        _reason: Option<&str>,
    ) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn kick_member(
        &self,
        _user_id: serenity::all::UserId,
        _reason: Option<&str>,
    ) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn edit_member(
        &self,
        _user_id: serenity::all::UserId,
        _map: impl serde::Serialize,
        _audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::Member, khronos_runtime::Error> {
        todo!()
    }

    async fn create_message(
        &self,
        _channel_id: serenity::all::ChannelId,
        _files: Vec<serenity::all::CreateAttachment<'_>>,
        _data: impl serde::Serialize,
    ) -> Result<serenity::model::channel::Message, khronos_runtime::Error> {
        todo!()
    }

    async fn create_interaction_response(
        &self,
        _interaction_id: serenity::all::InteractionId,
        _interaction_token: &str,
        _response: impl serde::Serialize,
        _files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn create_followup_message(
        &self,
        _interaction_token: &str,
        _response: impl serde::Serialize,
        _files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<serenity::all::Message, khronos_runtime::Error> {
        todo!()
    }

    async fn get_original_interaction_response(
        &self,
        _interaction_token: &str,
    ) -> Result<serenity::model::channel::Message, khronos_runtime::Error> {
        todo!()
    }

    async fn get_guild_commands(
        &self,
    ) -> Result<Vec<serenity::all::Command>, khronos_runtime::Error> {
        todo!()
    }

    async fn get_guild_command(
        &self,
        _command_id: serenity::all::CommandId,
    ) -> Result<serenity::all::Command, khronos_runtime::Error> {
        todo!()
    }

    async fn create_guild_command(
        &self,
        _map: impl serde::Serialize,
    ) -> Result<serenity::all::Command, khronos_runtime::Error> {
        todo!()
    }

    async fn get_messages(
        &self,
        _channel_id: serenity::all::ChannelId,
        _target: Option<serenity::all::MessagePagination>,
        _limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> Result<Vec<serenity::all::Message>, khronos_runtime::Error> {
        todo!()
    }

    async fn get_message(
        &self,
        _channel_id: serenity::all::ChannelId,
        _message_id: serenity::all::MessageId,
    ) -> Result<serenity::all::Message, khronos_runtime::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct CliLockdownProvider {}

impl LockdownProvider for CliLockdownProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn list(
        &self,
    ) -> Result<Vec<khronos_runtime::traits::ir::Lockdown>, khronos_runtime::Error> {
        todo!()
    }

    async fn qsl(&self, _reason: String) -> Result<uuid::Uuid, khronos_runtime::Error> {
        todo!()
    }

    async fn tsl(&self, _reason: String) -> Result<uuid::Uuid, khronos_runtime::Error> {
        todo!()
    }

    async fn scl(
        &self,
        _channel_id: serenity::all::ChannelId,
        _reason: String,
    ) -> Result<uuid::Uuid, khronos_runtime::Error> {
        todo!()
    }

    async fn role(
        &self,
        _role_id: serenity::all::RoleId,
        _reason: String,
    ) -> Result<uuid::Uuid, khronos_runtime::Error> {
        todo!()
    }

    async fn remove(&self, _id: uuid::Uuid) -> Result<(), khronos_runtime::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct CliUserInfoProvider {}

impl UserInfoProvider for CliUserInfoProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn get(
        &self,
        _user_id: serenity::all::UserId,
    ) -> Result<UserInfo, khronos_runtime::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct CliStingProvider {}

impl StingProvider for CliStingProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn list(&self, _page: usize) -> Result<Vec<Sting>, khronos_runtime::Error> {
        todo!()
    }

    async fn get(&self, _id: uuid::Uuid) -> Result<Option<Sting>, khronos_runtime::Error> {
        todo!()
    }

    async fn create(&self, _sting: StingCreate) -> Result<uuid::Uuid, khronos_runtime::Error> {
        todo!()
    }

    async fn update(&self, _sting: Sting) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn delete(&self, _id: uuid::Uuid) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    /// Returns a StingAggregate set for a user in the guild
    async fn guild_user_aggregate(
        &self,
        _target: serenity::all::UserId,
    ) -> Result<Vec<StingAggregate>, khronos_runtime::Error> {
        todo!()
    }

    /// Returns a StingAggregate set for the guild
    async fn guild_aggregate(&self) -> Result<Vec<StingAggregate>, khronos_runtime::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct CliPageProvider {}

impl PageProvider for CliPageProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn get_page(&self) -> Option<khronos_runtime::traits::ir::Page> {
        todo!()
    }

    async fn set_page(
        &self,
        _page: khronos_runtime::traits::ir::Page,
    ) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn delete_page(&self) -> Result<(), khronos_runtime::Error> {
        todo!()
    }
}
