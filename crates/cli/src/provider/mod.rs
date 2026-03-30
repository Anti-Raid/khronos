use dapi::EVENT_LIST;
use dapi::controller::DiscordProviderContext;
use khronos_runtime::traits::httpclientprovider::HTTPClientProvider;
use khronos_runtime::traits::runtimeprovider::RuntimeProvider;
use moka::future::Cache;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;

use crate::constants::default_global_guild_id;
use khronos_runtime::traits::context::KhronosContext;
use dapi::controller::DiscordProvider;
use khronos_runtime::traits::ir::runtime as runtime_ir;

/// Internal short-lived channel cache
pub static CHANNEL_CACHE: LazyLock<Cache<serenity::all::GenericChannelId, Value>> =
    LazyLock::new(|| {
        Cache::builder()
            .time_to_idle(std::time::Duration::from_secs(30))
            .build()
    });

#[derive(Clone)]
pub struct CliKhronosContext {
    pub guild_id: Option<serenity::all::GuildId>,
    pub http: Option<Arc<serenity::all::Http>>,
}

impl KhronosContext for CliKhronosContext {
    type DiscordProvider = CliDiscordProvider;
    type HTTPClientProvider = CliHttpClientProvider;
    type RuntimeProvider = CliRuntimeProvider;

    fn discord_provider(&self) -> Option<Self::DiscordProvider> {
        let guild_id = if let Some(guild_id) = self.guild_id {
            guild_id
        } else {
            default_global_guild_id()
        };

        self.http.as_ref().map(|http| CliDiscordProvider {
            http: http.clone(),
            guild_id,
        })
    }

    fn httpclient_provider(&self) -> Option<Self::HTTPClientProvider> {
        Some(CliHttpClientProvider)
    }

    fn runtime_provider(&self) -> Option<Self::RuntimeProvider> {
        Some(CliRuntimeProvider {
        })
    }
}

#[derive(Clone)]
pub struct CliDiscordProvider {
    guild_id: serenity::all::GuildId,
    http: Arc<serenity::all::Http>,
}

impl DiscordProvider for CliDiscordProvider {
    fn attempt_action(&self, _bucket: &str) -> serenity::Result<(), khronos_runtime::Error> {
        Ok(())
    }

    fn current_user(&self) -> Option<serenity::all::CurrentUser> {
        None
    }

    async fn get_guild(
        &self,
    ) -> serenity::Result<Value, khronos_runtime::Error> {
        // Fetch from HTTP
        self.http
            .get_guild(self.guild_id)
            .await
            .map_err(|e| format!("Failed to fetch guild: {e}").into())
    }

    async fn get_channel(
        &self,
        channel_id: serenity::all::GenericChannelId,
    ) -> serenity::Result<Value, khronos_runtime::Error> {
        {
            // Check cache first
            let cached_channel = CHANNEL_CACHE.get(&channel_id).await;

            if let Some(cached_channel) = cached_channel {
                let Some(Value::String(guild_id)) = cached_channel.get("guild_id") else {
                    return Err(format!("Channel {channel_id} does not belong to a guild").into());
                };

                if guild_id != &self.guild_id.to_string() {
                    return Err(format!("Channel {channel_id} does not belong to the guild").into());
                }

                return Ok(cached_channel);
            }
        }

        // Fetch from HTTP
        let channel = self.http.get_channel(channel_id).await?;

        let Some(Value::String(guild_id)) = channel.get("guild_id") else {
            return Err(format!("Channel {channel_id} does not belong to a guild").into());
        };

        if guild_id != &self.guild_id.to_string() {
            return Err(format!("Channel {channel_id} does not belong to the guild").into());
        }

        Ok(channel)
    }

    fn context(&self) -> DiscordProviderContext {
        DiscordProviderContext::Guild(self.guild_id)
    }

    fn serenity_http(&self) -> &serenity::all::Http {
        &self.http
    }

    async fn edit_channel(
        &self,
        channel_id: serenity::all::GenericChannelId,
        map: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, khronos_runtime::Error> {
        let chan = self
            .http
            .edit_channel(channel_id, &map, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit channel: {e}"))?;

        // Update cache
        CHANNEL_CACHE.insert(channel_id, chan.clone()).await;

        Ok(chan)
    }

    async fn delete_channel(
        &self,
        channel_id: serenity::all::GenericChannelId,
        audit_log_reason: Option<&str>,
    ) -> Result<Value, khronos_runtime::Error> {
        let chan = self
            .http
            .delete_channel(channel_id, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to delete channel: {e}"))?;

        // Remove from cache
        CHANNEL_CACHE.remove(&channel_id).await;

        Ok(chan)
    }

    async fn edit_channel_permissions(
        &self,
        channel_id: serenity::all::GenericChannelId,
        target_id: serenity::all::TargetId,
        data: impl serde::Serialize,
        audit_log_reason: Option<&str>,
    ) -> Result<(), khronos_runtime::Error> {
        self.http
            .create_permission(channel_id.expect_channel(), target_id, &data, audit_log_reason)
            .await
            .map_err(|e| format!("Failed to edit channel permissions: {e}"))?;

        CHANNEL_CACHE.remove(&channel_id).await;

        Ok(())
    }
}

#[derive(Clone)]
pub struct CliHttpClientProvider;

impl HTTPClientProvider for CliHttpClientProvider {
    fn allow_localhost(&self) -> bool {
        false // CLI mode does not allow localhost access for testing purposes
    }

    fn attempt_action(&self, _bucket: &str, _url: &str) -> Result<(), khronos_runtime::Error> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct CliRuntimeProvider {
}

// TODO: Actually implement this correctly, for now everything is a stub
impl RuntimeProvider for CliRuntimeProvider {
    type SyscallArgs = bool; // dummy
    type SyscallRet = bool; // dummy

    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        Ok(())
    }

    fn get_exposed_vfs(&self) -> Result<HashMap<String, khronos_runtime::core::typesext::Vfs>, khronos_runtime::Error> {
        // CLI mode does not expose any VFS mappings
        Ok(std::collections::HashMap::new())
    }

    async fn syscall(&self, _ops: bool) -> Result<bool, khronos_runtime::Error> {
        Err("Not supported".into())
    }

    async fn stats(&self) -> Result<runtime_ir::RuntimeStats, khronos_runtime::Error> {
        // TODO: Support customizing this to smth sensible
        Ok(runtime_ir::RuntimeStats {
            total_cached_guilds: 0,
            total_guilds: 1,
            total_users: 1,
            last_started_at: chrono::Utc::now(),
        })
    }

    fn links(&self) -> Result<runtime_ir::RuntimeLinks, khronos_runtime::Error> {
        // TODO: Support customizing this to smth sensible
        Ok(runtime_ir::RuntimeLinks {
            support_server: "cli".to_string(),
            api_url: "cli".to_string(),
            frontend_url: "cli".to_string(),
            docs_url: "cli".to_string()
        })
    }

    fn event_list(&self) -> Result<Vec<String>, khronos_runtime::Error> {
        Ok(EVENT_LIST
            .iter()
            .copied()
            .map(|x| x.to_string())
            .collect::<Vec<String>>())
    }
}