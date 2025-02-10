use moka::future::Cache;
use serenity::all::InteractionId;
use std::rc::Rc;
use std::sync::LazyLock;

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

use crate::cli::CliAuxOpts;
use crate::constants::default_global_guild_id;
use crate::filestorage::FileStorageProvider;

/// Internal short-lived channel cache
pub static CHANNEL_CACHE: LazyLock<Cache<serenity::all::ChannelId, serenity::all::GuildChannel>> =
    LazyLock::new(|| {
        Cache::builder()
            .time_to_idle(std::time::Duration::from_secs(30))
            .build()
    });

#[derive(Clone)]
pub struct CliKhronosContext {
    pub data: serde_json::Value,
    pub file_storage_provider: Option<Rc<dyn FileStorageProvider>>,
    pub aux_opts: CliAuxOpts,
    pub allowed_caps: Vec<String>,
    pub guild_id: Option<serenity::all::GuildId>,
    pub owner_guild_id: Option<serenity::all::GuildId>,
    pub global_table: mlua::Table,
    pub http: Option<Rc<serenity::all::Http>>,
    pub cache: Option<Rc<serenity::cache::Cache>>,
}

impl KhronosContext for CliKhronosContext {
    type Data = serde_json::Value;
    type KVProvider = CliKVProvider;
    type DiscordProvider = CliDiscordProvider;
    type LockdownProvider = CliLockdownProvider;
    type UserInfoProvider = CliUserInfoProvider;
    type StingProvider = CliStingProvider;
    type PageProvider = CliPageProvider;

    fn data(&self) -> Self::Data {
        if self.data == serde_json::Value::Null {
            let val =
                serde_json::to_value(&self.aux_opts).expect("Failed to serialize aux_opts to JSON");

            return val;
        }
        self.data.clone()
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
        self.cache.as_ref().map(|c| c.current_user().clone())
    }

    /// Returns the global table to use
    fn global_table(&self) -> mlua::Table {
        self.global_table.clone()
    }

    fn kv_provider(&self, scope: ExecutorScope) -> Option<Self::KVProvider> {
        if let Some(file_storage_provider) = &self.file_storage_provider {
            let guild_id = match scope {
                ExecutorScope::ThisGuild => self.guild_id?,
                ExecutorScope::OwnerGuild => {
                    if let Some(owner_guild_id) = self.owner_guild_id {
                        owner_guild_id
                    } else if let Some(guild_id) = self.guild_id {
                        guild_id
                    } else {
                        default_global_guild_id()
                    }
                }
            };

            Some(CliKVProvider {
                guild_id,
                file_storage_provider: file_storage_provider.clone(),
            })
        } else {
            None
        }
    }

    fn discord_provider(&self, scope: ExecutorScope) -> Option<Self::DiscordProvider> {
        if let Some(http) = &self.http {
            let guild_id = match scope {
                ExecutorScope::ThisGuild => self.guild_id?,
                ExecutorScope::OwnerGuild => {
                    if let Some(owner_guild_id) = self.owner_guild_id {
                        owner_guild_id
                    } else if let Some(guild_id) = self.guild_id {
                        guild_id
                    } else {
                        default_global_guild_id()
                    }
                }
            };
            Some(CliDiscordProvider {
                http: http.clone(),
                cache: self.cache.clone(),
                guild_id,
            })
        } else {
            None
        }
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
/// Represents a full record complete with metadata
pub struct CliKvRecord {
    pub key: String,
    pub value: serde_json::Value,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Clone)]
pub struct CliKVProvider {
    pub guild_id: serenity::all::GuildId,
    pub file_storage_provider: Rc<dyn FileStorageProvider>,
}

impl KVProvider for CliKVProvider {
    async fn get(
        &self,
        key: String,
    ) -> Result<Option<khronos_runtime::traits::ir::KvRecord>, khronos_runtime::Error> {
        let file_contents = self
            .file_storage_provider
            .get_file(&["keys".to_string(), self.guild_id.to_string(), key])
            .await
            .map_err(|e| format!("Failed to get file: {}", e))?;

        if file_contents.is_empty() {
            Ok(None)
        } else {
            let record: CliKvRecord = serde_json::from_slice(&file_contents)
                .map_err(|e| format!("Failed to parse record: {}", e))?;

            Ok(Some(khronos_runtime::traits::ir::KvRecord {
                key: record.key,
                value: record.value,
                created_at: record.created_at,
                last_updated_at: record.last_updated_at,
            }))
        }
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
        Ok(vec![])
    }

    async fn exists(&self, _key: String) -> Result<bool, khronos_runtime::Error> {
        todo!()
    }

    async fn keys(&self) -> Result<Vec<String>, khronos_runtime::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct CliDiscordProvider {
    guild_id: serenity::all::GuildId,
    http: Rc<serenity::all::Http>,
    cache: Option<Rc<serenity::cache::Cache>>,
}

impl DiscordProvider for CliDiscordProvider {
    fn attempt_action(&self, _bucket: &str) -> serenity::Result<(), khronos_runtime::Error> {
        Ok(())
    }

    async fn guild(
        &self,
    ) -> serenity::Result<serenity::model::prelude::PartialGuild, khronos_runtime::Error> {
        {
            if let Some(cache) = &self.cache {
                if let Some(guild) = cache.guild(self.guild_id) {
                    return Ok(guild.clone().into());
                }
            }
        }

        // Fetch from HTTP
        self.http
            .get_guild(self.guild_id)
            .await
            .map_err(|e| format!("Failed to fetch guild: {}", e).into())
    }

    async fn member(
        &self,
        user_id: serenity::all::UserId,
    ) -> serenity::Result<Option<serenity::all::Member>, khronos_runtime::Error> {
        {
            if let Some(cache) = &self.cache {
                if let Some(guild) = cache.guild(self.guild_id) {
                    if let Some(member) = guild.members.get(&user_id).cloned() {
                        return Ok(Some(member));
                    }
                }
            }
        }

        // Fetch from HTTP
        self.http
            .get_member(self.guild_id, user_id)
            .await
            .map_err(|e| format!("Failed to fetch member: {}", e).into())
            .map(Some)
    }

    async fn guild_channel(
        &self,
        channel_id: serenity::all::ChannelId,
    ) -> serenity::Result<serenity::all::GuildChannel, khronos_runtime::Error> {
        {
            // Check cache first
            let cached_channel = CHANNEL_CACHE.get(&channel_id).await;

            if let Some(cached_channel) = cached_channel {
                if cached_channel.guild_id != self.guild_id {
                    return Err("Channel not in guild".into());
                }

                return Ok(cached_channel);
            }
        }

        // Fetch from HTTP
        let channel = self.http.get_channel(channel_id).await?;

        let Some(guild_channel) = channel.guild() else {
            return Err("Channel not in guild".into());
        };

        if guild_channel.guild_id != self.guild_id {
            return Err("Channel not in guild".into());
        }

        Ok(guild_channel)
    }

    async fn get_audit_logs(
        &self,
        action_type: Option<serenity::all::audit_log::Action>,
        user_id: Option<serenity::model::prelude::UserId>,
        before: Option<serenity::model::prelude::AuditLogEntryId>,
        limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> Result<serenity::model::prelude::AuditLogs, khronos_runtime::Error> {
        self.http
            .get_audit_logs(self.guild_id, action_type, user_id, before, limit)
            .await
            .map_err(|e| format!("Failed to fetch audit logs: {}", e).into())
    }

    async fn get_automod_rules(
        &self,
    ) -> Result<Vec<serenity::model::guild::automod::Rule>, khronos_runtime::Error> {
        self.http
            .get_automod_rules(self.guild_id)
            .await
            .map_err(|e| format!("Failed to fetch automod rules: {}", e).into())
    }

    async fn get_automod_rule(
        &self,
        rule_id: serenity::all::RuleId,
    ) -> Result<serenity::model::guild::automod::Rule, khronos_runtime::Error> {
        self.http
            .get_automod_rule(self.guild_id, rule_id)
            .await
            .map_err(|e| format!("Failed to fetch automod rule: {}", e).into())
    }

    async fn edit_channel(
        &self,
        channel_id: serenity::all::ChannelId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::GuildChannel, khronos_runtime::Error> {
        let chan = self
            .http
            .edit_channel(channel_id, &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit channel: {}", e))?;

        // Update cache
        CHANNEL_CACHE.insert(channel_id, chan.clone()).await;

        Ok(chan)
    }

    async fn delete_channel(
        &self,
        channel_id: serenity::all::ChannelId,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::model::channel::Channel, khronos_runtime::Error> {
        let chan = self
            .http
            .delete_channel(channel_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to delete channel: {}", e))?;

        // Remove from cache
        CHANNEL_CACHE.remove(&channel_id).await;

        Ok(chan)
    }

    async fn create_member_ban(
        &self,
        user_id: serenity::all::UserId,
        delete_message_seconds: u32,
        reason: Option<&str>,
    ) -> Result<(), khronos_runtime::Error> {
        self.http
            .ban_user(
                self.guild_id,
                user_id,
                (delete_message_seconds / 86400)
                    .try_into()
                    .map_err(|e| format!("Failed to convert ban duration to days: {}", e))?,
                reason,
            )
            .await
            .map_err(|e| format!("Failed to ban user: {}", e).into())
    }

    async fn kick_member(
        &self,
        user_id: serenity::all::UserId,
        reason: Option<&str>,
    ) -> Result<(), khronos_runtime::Error> {
        self.http
            .kick_member(self.guild_id, user_id, reason)
            .await
            .map_err(|e| format!("Failed to kick user: {}", e).into())
    }

    async fn edit_member(
        &self,
        user_id: serenity::all::UserId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<serenity::all::Member, khronos_runtime::Error> {
        self.http
            .edit_member(self.guild_id, user_id, &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit member: {}", e).into())
    }

    async fn create_message(
        &self,
        channel_id: serenity::all::ChannelId,
        files: Vec<serenity::all::CreateAttachment<'_>>,
        data: impl serde::Serialize,
    ) -> Result<serenity::model::channel::Message, khronos_runtime::Error> {
        self.http
            .send_message(channel_id, files, &data)
            .await
            .map_err(|e| format!("Failed to send message: {}", e).into())
    }

    async fn create_interaction_response(
        &self,
        interaction_id: InteractionId,
        interaction_token: &str,
        response: impl serde::Serialize,
        files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<(), khronos_runtime::Error> {
        self.http
            .create_interaction_response(interaction_id, interaction_token, &response, files)
            .await
            .map_err(|e| format!("Failed to create interaction response: {}", e).into())
    }

    async fn create_followup_message(
        &self,
        interaction_token: &str,
        response: impl serde::Serialize,
        files: Vec<serenity::all::CreateAttachment<'_>>,
    ) -> Result<serenity::all::Message, khronos_runtime::Error> {
        self.http
            .create_followup_message(interaction_token, &response, files)
            .await
            .map_err(|e| format!("Failed to create interaction followup: {}", e).into())
    }

    async fn get_original_interaction_response(
        &self,
        interaction_token: &str,
    ) -> Result<serenity::model::channel::Message, khronos_runtime::Error> {
        self.http
            .get_original_interaction_response(interaction_token)
            .await
            .map_err(|e| format!("Failed to get original interaction response: {}", e).into())
    }

    async fn get_guild_commands(
        &self,
    ) -> Result<Vec<serenity::all::Command>, khronos_runtime::Error> {
        self.http
            .get_guild_commands(self.guild_id)
            .await
            .map_err(|e| format!("Failed to get guild commands: {}", e).into())
    }

    async fn get_guild_command(
        &self,
        command_id: serenity::all::CommandId,
    ) -> Result<serenity::all::Command, khronos_runtime::Error> {
        self.http
            .get_guild_command(self.guild_id, command_id)
            .await
            .map_err(|e| format!("Failed to get guild command: {}", e).into())
    }

    async fn create_guild_command(
        &self,
        map: impl serde::Serialize,
    ) -> Result<serenity::all::Command, khronos_runtime::Error> {
        self.http
            .create_guild_command(self.guild_id, &map)
            .await
            .map_err(|e| format!("Failed to create guild command: {}", e).into())
    }

    async fn get_messages(
        &self,
        channel_id: serenity::all::ChannelId,
        target: Option<serenity::all::MessagePagination>,
        limit: Option<serenity::nonmax::NonMaxU8>,
    ) -> Result<Vec<serenity::all::Message>, khronos_runtime::Error> {
        self.http
            .get_messages(channel_id, target, limit)
            .await
            .map_err(|e| format!("Failed to get messages: {}", e).into())
    }

    async fn get_message(
        &self,
        channel_id: serenity::all::ChannelId,
        message_id: serenity::all::MessageId,
    ) -> Result<serenity::all::Message, khronos_runtime::Error> {
        self.http
            .get_message(channel_id, message_id)
            .await
            .map_err(|e| format!("Failed to get message: {}", e).into())
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

#[allow(dead_code)]
/// Returns true if the filename matches the pattern based on PostgreSQL ILIKE pattern matching rules
fn does_file_match_pattern(filename: &str, pattern: &str) -> Result<bool, khronos_runtime::Error> {
    // An underscore (_) in pattern stands for (matches) any single character; a percent sign (%) matches any sequence of zero or more characters.
    let pattern = pattern.replace("_", ".").replace("%", ".*");
    let regex = regex::Regex::new(&format!("(?i)^{}$", pattern))?;
    Ok(regex.is_match(filename))
}

#[cfg(test)]
mod file_pattern_matching_tests {
    use super::does_file_match_pattern;

    #[test]
    fn test_does_file_match_pattern() {
        assert!(
            does_file_match_pattern("test", "test").unwrap(),
            "test should match test"
        );

        // Postgres docs cases
        assert!(
            does_file_match_pattern("abc", "abc").unwrap(),
            "abc should match abc"
        );
        assert!(
            does_file_match_pattern("abc", "a%").unwrap(),
            "abc should match a%"
        );
        assert!(
            does_file_match_pattern("abc", "%a%").unwrap(),
            "abc should match %a%"
        );
        assert!(
            does_file_match_pattern("abcde", "%c%").unwrap(),
            "abcde should match %c%"
        );
        assert!(
            does_file_match_pattern("abc", "_b_").unwrap(),
            "abc should match _b_"
        );
        assert!(
            !does_file_match_pattern("abc", "c").unwrap(),
            "abc should not match c"
        );

        assert!(
            does_file_match_pattern("abc", "a_c").unwrap(),
            "abc should match a_c"
        );
    }
}
