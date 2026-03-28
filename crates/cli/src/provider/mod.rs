use dapi::EVENT_LIST;
use dapi::controller::DiscordProviderContext;
use khronos_runtime::traits::httpclientprovider::HTTPClientProvider;
use khronos_runtime::traits::runtimeprovider::RuntimeProvider;
use moka::future::Cache;
use serde_json::Value;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::LazyLock;

use crate::constants::default_global_guild_id;
use crate::filestorage::FileStorageProvider;
use khronos_runtime::traits::context::KhronosContext;
use dapi::controller::DiscordProvider;
use khronos_runtime::traits::objectstorageprovider::ObjectStorageProvider;
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
    pub file_storage_provider: Rc<dyn FileStorageProvider>,
    pub guild_id: Option<serenity::all::GuildId>,
    pub http: Option<Arc<serenity::all::Http>>,
}

impl KhronosContext for CliKhronosContext {
    type DiscordProvider = CliDiscordProvider;
    type ObjectStorageProvider = CliObjectStorageProvider;
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

    fn objectstorage_provider(&self) -> Option<Self::ObjectStorageProvider> {
        Some(CliObjectStorageProvider {
            file_storage_provider: self.file_storage_provider.clone(),
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
pub struct CliObjectStorageProvider {
    pub file_storage_provider: Rc<dyn FileStorageProvider>,
}

impl ObjectStorageProvider for CliObjectStorageProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        Ok(())
    }

    fn bucket_name(&self) -> String {
        "cli".to_string()
    }

    async fn list_files(
        &self,
        prefix: Option<String>,
    ) -> Result<Vec<khronos_runtime::traits::ir::ObjectMetadata>, khronos_runtime::Error> {
        let prefix_split = prefix
            .as_ref()
            .map(|p| p.split('/').map(|s| s.to_string()).collect::<Vec<_>>())
            .unwrap_or_default();
        let mut fsp_vec = vec!["objects".to_string()];
        for prefix in prefix_split.into_iter() {
            fsp_vec.push(prefix);
        }
        let files = self
            .file_storage_provider
            .list_files(&fsp_vec, None)
            .await?;

        let mut objects = Vec::new();

        /*
                pub key: String,
        pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
        pub size: i64,
        pub etag: Option<String>,
            */

        for file in files {
            let object = khronos_runtime::traits::ir::ObjectMetadata {
                key: file.name.clone(),
                size: file.size as i64,
                last_modified: Some(file.last_updated_at),
                etag: format!("{}.{}.{}", file.name, file.created_at, file.last_updated_at).into(),
            };
            objects.push(object);
        }

        Ok(objects)
    }

    async fn file_exists(&self, key: String) -> Result<bool, khronos_runtime::Error> {
        // Split the key by '/' and add each part but last to fsp_vec
        let key_split = key.split('/').map(|s| s.to_string()).collect::<Vec<_>>();

        let key = key_split.last().unwrap_or(&"".to_string()).to_string();

        let mut fsp_vec = vec!["objects".to_string()];

        let key_split_len = key_split.len();
        for (i, prefix) in key_split.into_iter().enumerate() {
            if i == key_split_len - 1 {
                break;
            }
            fsp_vec.push(prefix);
        }

        self.file_storage_provider.file_exists(&fsp_vec, &key).await
    }

    async fn download_file(&self, key: String) -> Result<Vec<u8>, khronos_runtime::Error> {
        // Split the key by '/' and add each part but last to fsp_vec
        let key_split = key.split('/').map(|s| s.to_string()).collect::<Vec<_>>();

        // Get the key itself first
        let key = key_split.last().unwrap_or(&"".to_string()).to_string();

        let mut fsp_vec = vec!["objects".to_string()];

        let key_split_len = key_split.len();
        for (i, prefix) in key_split.into_iter().enumerate() {
            if i == key_split_len - 1 {
                break;
            }
            fsp_vec.push(prefix);
        }

        self.file_storage_provider
            .get_file(&fsp_vec, &key)
            .await?
            .map(|file| file.contents)
            .ok_or("Failed to download file".into())
    }

    async fn get_file_url(
        &self,
        key: String,
        expiry: std::time::Duration,
    ) -> Result<String, khronos_runtime::Error> {
        let base_path = self.file_storage_provider.base_path();
        Ok(format!(
            "file://{}/{}?expiry={}",
            base_path.display(),
            key,
            expiry.as_secs()
        ))
    }

    async fn upload_file(&self, key: String, data: Vec<u8>) -> Result<(), khronos_runtime::Error> {
        // Split the key by '/' and add each part but last to fsp_vec
        let key_split = key.split('/').map(|s| s.to_string()).collect::<Vec<_>>();

        let key = key_split.last().unwrap_or(&"".to_string()).to_string();

        let mut fsp_vec = vec!["objects".to_string()];

        let key_split_len = key_split.len();
        for (i, prefix) in key_split.into_iter().enumerate() {
            if i == key_split_len - 1 {
                break;
            }
            fsp_vec.push(prefix);
        }

        self.file_storage_provider
            .save_file(&fsp_vec, &key, &data)
            .await
    }

    async fn delete_file(&self, key: String) -> Result<(), khronos_runtime::Error> {
        // Split the key by '/' and add each part but last to fsp_vec
        let key_split = key.split('/').map(|s| s.to_string()).collect::<Vec<_>>();

        let key = key_split.last().unwrap_or(&"".to_string()).to_string();

        let mut fsp_vec = vec!["objects".to_string()];

        let key_split_len = key_split.len();
        for (i, prefix) in key_split.into_iter().enumerate() {
            if i == key_split_len - 1 {
                break;
            }
            fsp_vec.push(prefix);
        }

        self.file_storage_provider.delete_file(&fsp_vec, &key).await
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
    type StateOps = bool; // dummy
    type StateResult = bool; // dummy

    fn attempt_action(&self, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        Ok(())
    }

    fn get_exposed_vfs(&self) -> Result<HashMap<String, khronos_runtime::core::typesext::Vfs>, khronos_runtime::Error> {
        // CLI mode does not expose any VFS mappings
        Ok(std::collections::HashMap::new())
    }

    async fn state_op(&self, _ops: Vec<bool>) -> Result<Vec<bool>, khronos_runtime::Error> {
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