use moka::future::Cache;
use std::rc::Rc;
use std::sync::LazyLock;

use crate::cli::CliAuxOpts;
use crate::constants::default_global_guild_id;
use crate::filestorage::FileStorageProvider;
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
use khronos_runtime::rt::RuntimeShareableData;
use khronos_runtime::traits::scheduledexecprovider::ScheduledExecProvider;

/// Internal short-lived channel cache
pub static CHANNEL_CACHE: LazyLock<Cache<serenity::all::ChannelId, serenity::all::GuildChannel>> =
    LazyLock::new(|| {
        Cache::builder()
            .time_to_idle(std::time::Duration::from_secs(30))
            .build()
    });

#[derive(Clone)]
pub struct CliLockdownDataStore {}

impl lockdowns::LockdownDataStore for CliLockdownDataStore {
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


#[derive(Clone)]
pub struct CliScheduledExecProvider {}

impl ScheduledExecProvider<CliKhronosContext> for CliScheduledExecProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        Ok(())
    }

    async fn list(
        &self,
        _context: &CliKhronosContext,
    ) -> Result<Vec<(String, serde_json::Value)>, khronos_runtime::Error> {
        todo!()
    }

    async fn add(
        &self,
        _id: String,
        _data: serde_json::Value,
        _run_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), khronos_runtime::Error> {
        todo!()
    }

    async fn remove(&self, _id: String) -> Result<(), khronos_runtime::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct CliKhronosContext {
    pub data: serde_json::Value,
    pub file_storage_provider: Rc<dyn FileStorageProvider>,
    pub aux_opts: CliAuxOpts,
    pub allowed_caps: Vec<String>,
    pub guild_id: Option<serenity::all::GuildId>,
    pub owner_guild_id: Option<serenity::all::GuildId>,
    pub runtime_shareable_data: RuntimeShareableData,
    pub http: Option<Rc<serenity::all::Http>>,
    pub cache: Option<Rc<serenity::cache::Cache>>,
}

impl KhronosContext for CliKhronosContext {
    type Data = serde_json::Value;
    type KVProvider = CliKVProvider;
    type DiscordProvider = CliDiscordProvider;
    type LockdownDataStore = CliLockdownDataStore;
    type LockdownProvider = CliLockdownProvider;
    type UserInfoProvider = CliUserInfoProvider;
    type StingProvider = CliStingProvider;
    type PageProvider = CliPageProvider;
    type ScheduledExecProvider = CliScheduledExecProvider;

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

    /// Returns the runtime shareable data
    fn runtime_shareable_data(&self) -> khronos_runtime::rt::RuntimeShareableData {
        self.runtime_shareable_data.clone()
    }

    fn kv_provider(&self, scope: ExecutorScope) -> Option<Self::KVProvider> {
        let guild_id = match scope {
            ExecutorScope::ThisGuild => {
                if let Some(guild_id) = self.guild_id {
                    guild_id
                } else {
                    default_global_guild_id()
                }
            }
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
            file_storage_provider: self.file_storage_provider.clone(),
        })
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
        let Some(http) = &self.http else {
            return None;
        };

        Some(CliLockdownProvider {
            _lockdown_data_store: CliLockdownDataStore {},
            http: http.clone(),
        })
    }

    fn userinfo_provider(&self, _scope: ExecutorScope) -> Option<Self::UserInfoProvider> {
        Some(CliUserInfoProvider {})
    }

    fn sting_provider(&self, scope: ExecutorScope) -> Option<Self::StingProvider> {
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

        Some(CliStingProvider {
            guild_id,
            file_storage_provider: self.file_storage_provider.clone(),
        })
    }

    fn page_provider(&self, _scope: ExecutorScope) -> Option<Self::PageProvider> {
        Some(CliPageProvider {})
    }
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
        let Some(file_contents) = self
            .file_storage_provider
            .get_file(&[self.guild_id.to_string(), "keys".to_string()], &key)
            .await
            .map_err(|e| format!("Failed to get file: {}", e))?
        else {
            return Ok(None);
        };

        let record: serde_json::Value = serde_json::from_slice(&file_contents.contents)
            .map_err(|e| format!("Failed to parse record: {}", e))?;

        Ok(Some(khronos_runtime::traits::ir::KvRecord {
            key: file_contents.name,
            value: record,
            created_at: Some(file_contents.created_at),
            last_updated_at: Some(file_contents.last_updated_at),
        }))
    }

    async fn set(
        &self,
        key: String,
        value: serde_json::Value,
    ) -> Result<(), khronos_runtime::Error> {
        let value = serde_json::to_string(&value)
            .map_err(|e| format!("Failed to serialize value: {}", e))?;

        self.file_storage_provider
            .save_file(
                &[self.guild_id.to_string(), "keys".to_string()],
                &key,
                value.as_bytes(),
            )
            .await
    }

    async fn delete(&self, key: String) -> Result<(), khronos_runtime::Error> {
        self.file_storage_provider
            .delete_file(&[self.guild_id.to_string(), "keys".to_string()], &key)
            .await
    }

    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        Ok(())
    }

    async fn find(
        &self,
        query: String,
    ) -> Result<Vec<khronos_runtime::traits::ir::KvRecord>, khronos_runtime::Error> {
        let entries = self
            .file_storage_provider
            .list_files(
                &[self.guild_id.to_string(), "keys".to_string()],
                Some(query),
                None,
            )
            .await?;

        let mut records = Vec::new();
        for record in entries {
            let value: serde_json::Value = serde_json::from_slice(&record.contents)
                .map_err(|e| format!("Failed to parse record: {}", e))?;

            records.push(khronos_runtime::traits::ir::KvRecord {
                key: record.name,
                value,
                created_at: Some(record.created_at),
                last_updated_at: Some(record.last_updated_at),
            });
        }

        Ok(records)
    }

    async fn exists(&self, key: String) -> Result<bool, khronos_runtime::Error> {
        self.file_storage_provider
            .file_exists(&["keys".to_string(), self.guild_id.to_string()], &key)
            .await
    }

    async fn keys(&self) -> Result<Vec<String>, khronos_runtime::Error> {
        self.file_storage_provider
            .list_files(&["keys".to_string(), self.guild_id.to_string()], None, None)
            .await
            .map(|entries| entries.into_iter().map(|e| e.name).collect())
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

    fn guild_id(&self) -> serenity::all::GuildId {
        self.guild_id
    }

    fn serenity_http(&self) -> &serenity::all::Http {
        &self.http
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

    async fn edit_channel_permissions(
        &self,
        channel_id: serenity::all::ChannelId,
        target_id: serenity::all::TargetId,
        data: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<(), khronos_runtime::Error> {
        self.http
            .create_permission(channel_id, target_id, &data, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit channel permissions: {}", e))?;

        CHANNEL_CACHE.remove(&channel_id).await;

        Ok(())
    }
}

#[derive(Clone)]
pub struct CliLockdownProvider {
    _lockdown_data_store: CliLockdownDataStore,
    http: Rc<serenity::all::Http>,
}

impl LockdownProvider<CliLockdownDataStore> for CliLockdownProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        Ok(())
    }

    fn lockdown_data_store(&self) -> &CliLockdownDataStore {
        &self._lockdown_data_store
    }

    fn serenity_http(&self) -> &serenity::http::Http {
        &self.http
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

#[allow(dead_code)] // TODO: Implement
#[derive(Clone)]
pub struct CliStingProvider {
    pub guild_id: serenity::all::GuildId,
    pub file_storage_provider: Rc<dyn FileStorageProvider>,
}

impl StingProvider for CliStingProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        Ok(())
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
