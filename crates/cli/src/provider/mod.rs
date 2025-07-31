use khronos_runtime::traits::context::CompatibilityFlags;
use khronos_runtime::traits::context::Limitations;
use khronos_runtime::traits::httpclientprovider::HTTPClientProvider;
use khronos_runtime::traits::httpserverprovider::HTTPServerProvider;
use moka::future::Cache;
use serde_json::Value;
use sqlx::Row;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::LazyLock;

use crate::constants::default_global_guild_id;
use crate::filestorage::FileStorageProvider;
use khronos_runtime::traits::context::KhronosContext;
use khronos_runtime::traits::context::ScriptData;
use khronos_runtime::traits::datastoreprovider::{DataStoreImpl, DataStoreProvider};
use khronos_runtime::traits::discordprovider::DiscordProvider;
use khronos_runtime::traits::kvprovider::KVProvider;
use khronos_runtime::traits::objectstorageprovider::ObjectStorageProvider;

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
    pub allowed_caps: Vec<String>,
    pub guild_id: Option<serenity::all::GuildId>,
    pub owner_guild_id: Option<serenity::all::GuildId>,
    pub http: Option<Arc<serenity::all::Http>>,
    pub template_name: String,
    pub script_data: ScriptData,
    pub pool: Option<sqlx::PgPool>,
}

pub(crate) fn default_script_data(allowed_caps: Vec<String>) -> ScriptData {
    ScriptData {
        guild_id: None,
        name: "cli".to_string(),
        description: None,
        shop_name: None,
        shop_owner: None,
        events: vec![],
        error_channel: None,
        lang: "luau".to_string(),
        allowed_caps,
        compatibility_flags: CompatibilityFlags::empty(),
        created_by: None,
        created_at: None,
        updated_by: None,
        updated_at: None,
    }
}

impl KhronosContext for CliKhronosContext {
    type KVProvider = CliKVProvider;
    type DiscordProvider = CliDiscordProvider;
    type DataStoreProvider = CliDataStoreProvider;
    type ObjectStorageProvider = CliObjectStorageProvider;
    type HTTPClientProvider = CliHttpClientProvider;
    type HTTPServerProvider = CliHttpServerProvider;

    fn data(&self) -> &ScriptData {
        &self.script_data
    }

    fn limitations(&self) -> Limitations {
        Limitations::new(self.allowed_caps.clone())
    }

    fn guild_id(&self) -> Option<serenity::all::GuildId> {
        self.guild_id
    }

    fn owner_guild_id(&self) -> Option<serenity::all::GuildId> {
        self.owner_guild_id
    }

    fn template_name(&self) -> String {
        self.template_name.clone()
    }

    fn current_user(&self) -> Option<serenity::all::CurrentUser> {
        None // CLI mode does not have a current user yet
    }

    fn kv_provider(&self) -> Option<Self::KVProvider> {
        let guild_id = if let Some(guild_id) = self.guild_id {
            guild_id
        } else {
            default_global_guild_id()
        };

        let Some(pool) = &self.pool else {
            eprintln!("WARNING: A postgres pool is required for KVProvider in CLI mode.");
            return None;
        };

        Some(CliKVProvider {
            guild_id,
            pool: pool.clone(),
        })
    }

    fn datastore_provider(&self) -> Option<Self::DataStoreProvider> {
        let guild_id = if let Some(guild_id) = self.guild_id {
            guild_id
        } else {
            default_global_guild_id()
        };

        Some(CliDataStoreProvider { guild_id })
    }

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

    fn httpserver_provider(&self) -> Option<Self::HTTPServerProvider> {
        Some(CliHttpServerProvider)
    }
}

#[derive(Clone)]
pub struct CliKVProvider {
    pub guild_id: serenity::all::GuildId,
    pub pool: sqlx::PgPool,
}

impl KVProvider for CliKVProvider {
    async fn list_scopes(&self) -> Result<Vec<String>, khronos_runtime::Error> {
        let query = sqlx::query(
            "SELECT DISTINCT unnest_scope AS scope
FROM kv_v2, unnest(scopes) AS unnest_scope
ORDER BY scope;
        ",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to list scopes: {e}"))?
        .iter()
        .map(|row| row.get::<String, _>("scope"))
        .collect::<Vec<_>>();

        Ok(query)
    }

    async fn get(
        &self,
        scopes: &[String],
        key: String,
    ) -> Result<Option<khronos_runtime::traits::ir::KvRecord>, khronos_runtime::Error> {
        let Some(data) = sqlx::query(
            "SELECT id, key, value, created_at, last_updated_at, scopes, expires_at, resume
            FROM kv_v2
            WHERE 
            guild_id = $1 AND
            key = $2 AND
            scopes @> $3
        ",
        )
        .bind(self.guild_id.to_string())
        .bind(key)
        .bind(scopes)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to get key: {e}"))?
        else {
            return Ok(None);
        };

        let value = data.get::<serde_json::Value, _>("value");

        let record: khronos_runtime::utils::khronos_value::KhronosValue = value.try_into()?;

        let file_contents = khronos_runtime::traits::ir::KvRecord {
            id: data.get::<sqlx::types::uuid::Uuid, _>("id").to_string(),
            key: data.get::<String, _>("key"),
            value: record,
            created_at: Some(data.get::<chrono::DateTime<chrono::Utc>, _>("created_at")),
            last_updated_at: Some(data.get::<chrono::DateTime<chrono::Utc>, _>("last_updated_at")),
            scopes: data.get::<Vec<String>, _>("scopes"),
            expires_at: data.get::<Option<chrono::DateTime<chrono::Utc>>, _>("expires_at"),
            resume: data.get::<bool, _>("resume"),
        };

        Ok(Some(file_contents))
    }

    async fn get_by_id(
        &self,
        id: String,
    ) -> Result<Option<khronos_runtime::traits::ir::KvRecord>, khronos_runtime::Error> {
        let Some(data) = sqlx::query(
            "SELECT id, key, value, created_at, last_updated_at, scopes, expires_at, resume
            FROM kv_v2
            WHERE 
            guild_id = $1 AND
            id = $2
        ",
        )
        .bind(self.guild_id.to_string())
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to get key: {e}"))?
        else {
            return Ok(None);
        };

        let value = data.get::<serde_json::Value, _>("value");

        let record: khronos_runtime::utils::khronos_value::KhronosValue = value.try_into()?;

        let file_contents = khronos_runtime::traits::ir::KvRecord {
            id: data.get::<sqlx::types::uuid::Uuid, _>("id").to_string(),
            key: data.get::<String, _>("key"),
            value: record,
            created_at: Some(data.get::<chrono::DateTime<chrono::Utc>, _>("created_at")),
            last_updated_at: Some(data.get::<chrono::DateTime<chrono::Utc>, _>("last_updated_at")),
            scopes: data.get::<Vec<String>, _>("scopes"),
            expires_at: data.get::<Option<chrono::DateTime<chrono::Utc>>, _>("expires_at"),
            resume: data.get::<bool, _>("resume"),
        };

        Ok(Some(file_contents))
    }

    async fn set(
        &self,
        scopes: &[String],
        key: String,
        value: khronos_runtime::utils::khronos_value::KhronosValue,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
        resume: bool,
    ) -> Result<(bool, String), khronos_runtime::Error> {
        if let Some(existing) = sqlx::query(
            "SELECT id
            FROM kv_v2
            WHERE 
            guild_id = $1 AND
            key = $2 AND
            scopes @> $3
        ",
        )
        .bind(self.guild_id.to_string())
        .bind(&key)
        .bind(scopes)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to get existing keys: {e}"))?
        {
            let key = existing.get::<sqlx::types::uuid::Uuid, _>("id");
            sqlx::query("UPDATE kv_v2 SET value = $1, expires_at = $2, resume = $3 WHERE id = $4")
                .bind(value.into_serde_json_value(1, true)?)
                .bind(expires_at)
                .bind(resume)
                .bind(key)
                .execute(&self.pool)
                .await
                .map_err(|e| format!("Failed to set key: {e}"))?;

            return Ok((true, key.to_string()));
        }

        let id = sqlx::query("INSERT INTO kv_v2 (guild_id, key, value, scopes, expires_at, resume) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
            .bind(self.guild_id.to_string())
            .bind(&key)
            .bind(value.into_serde_json_value(1, true)?)
            .bind(scopes)
            .bind(expires_at)
            .bind(resume)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Failed to set key: {e}"))?
            .try_get::<sqlx::types::uuid::Uuid, _>("id")
            .map_err(|e| format!("Failed to get ID: {e}"))?;

        Ok((false, id.to_string()))
    }

    async fn set_by_id(
        &self,
        id: String,
        value: khronos_runtime::utils::khronos_value::KhronosValue,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
        resume: bool,
    ) -> Result<(), khronos_runtime::Error> {
        let key = sqlx::types::uuid::Uuid::parse_str(&id)
            .map_err(|e| format!("Failed to parse ID: {e}"))?;

        sqlx::query("UPDATE kv_v2 SET value = $1, expires_at = $2, resume = $3 WHERE id = $4")
            .bind(value.into_serde_json_value(1, true)?)
            .bind(expires_at)
            .bind(resume)
            .bind(key)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to set key: {e}"))?;

        Ok(())
    }

    async fn set_expiry(
        &self,
        scopes: &[String],
        key: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), khronos_runtime::Error> {
        sqlx::query(
            "UPDATE kv_v2
            SET expires_at = $1
            WHERE 
            guild_id = $2 AND
            key = $3 AND
            scopes @> $4
        ",
        )
        .bind(expires_at)
        .bind(self.guild_id.to_string())
        .bind(key)
        .bind(scopes)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to set expiry: {e}"))?;

        Ok(())
    }

    async fn set_expiry_by_id(
        &self,
        id: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), khronos_runtime::Error> {
        let key = sqlx::types::uuid::Uuid::parse_str(&id)
            .map_err(|e| format!("Failed to parse ID: {e}"))?;

        sqlx::query(
            "UPDATE kv_v2
            SET expires_at = $1
            WHERE
            guild_id = $2 AND
            id = $3
        ",
        )
        .bind(expires_at)
        .bind(self.guild_id.to_string())
        .bind(key)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to set expiry: {e}"))?;

        Ok(())
    }

    async fn delete(&self, scopes: &[String], key: String) -> Result<(), khronos_runtime::Error> {
        sqlx::query(
            "DELETE FROM kv_v2
            WHERE 
            guild_id = $1 AND
            key = $2 AND
            scopes @> $3
        ",
        )
        .bind(self.guild_id.to_string())
        .bind(key)
        .bind(scopes)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to get key: {e}"))?;

        Ok(())
    }

    async fn delete_by_id(&self, id: String) -> Result<(), khronos_runtime::Error> {
        sqlx::query(
            "DELETE FROM kv_v2
            WHERE 
            guild_id = $1 AND
            id = $2
        ",
        )
        .bind(self.guild_id.to_string())
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to get key: {e}"))?;

        Ok(())
    }

    fn attempt_action(
        &self,
        _scopes: &[String],
        _bucket: &str,
    ) -> Result<(), khronos_runtime::Error> {
        Ok(())
    }

    async fn find(
        &self,
        scopes: &[String],
        query: String,
    ) -> Result<Vec<khronos_runtime::traits::ir::KvRecord>, khronos_runtime::Error> {
        let entries = if query == "%%" {
            // Fast path for querying all keys
            sqlx::query(
                "SELECT id, key, value, created_at, last_updated_at, scopes, expires_at
                FROM kv_v2
                WHERE 
                guild_id = $1 
                AND scopes @> $2
            ",
            )
            .bind(self.guild_id.to_string())
            .bind(scopes)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to get key: {e}"))?
        } else {
            sqlx::query(
                "SELECT id, key, value, created_at, last_updated_at, scopes, expires_at, resume
                FROM kv_v2
                WHERE 
                guild_id = $1 
                AND key ILIKE $2
                AND scopes @> $3
            ",
            )
            .bind(self.guild_id.to_string())
            .bind(query)
            .bind(scopes)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to get key: {e}"))?
        };

        let mut records = Vec::new();
        for data in entries {
            let value = data.get::<serde_json::Value, _>("value");

            let record: khronos_runtime::utils::khronos_value::KhronosValue = value.try_into()?;

            let file_contents = khronos_runtime::traits::ir::KvRecord {
                id: data.get::<sqlx::types::uuid::Uuid, _>("id").to_string(),
                key: data.get::<String, _>("key"),
                value: record,
                created_at: Some(data.get::<chrono::DateTime<chrono::Utc>, _>("created_at")),
                last_updated_at: Some(
                    data.get::<chrono::DateTime<chrono::Utc>, _>("last_updated_at"),
                ),
                scopes: data.get::<Vec<String>, _>("scopes"),
                expires_at: data.get::<Option<chrono::DateTime<chrono::Utc>>, _>("expires_at"),
                resume: data.get::<bool, _>("resume"),
            };

            records.push(file_contents);
        }

        Ok(records)
    }

    async fn exists(&self, scopes: &[String], key: String) -> Result<bool, khronos_runtime::Error> {
        let data = sqlx::query(
            "SELECT COUNT(*)
            FROM kv_v2
            WHERE 
            guild_id = $1 
            AND key = $2
            AND scopes @> $3
            LIMIT 1
        ",
        )
        .bind(self.guild_id.to_string())
        .bind(key)
        .bind(scopes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to get key: {e}"))?;

        let count = data.get::<i64, _>(0);

        Ok(count > 0)
    }

    async fn keys(&self, scopes: &[String]) -> Result<Vec<String>, khronos_runtime::Error> {
        let data = sqlx::query(
            "SELECT key
            FROM kv_v2
            WHERE 
            guild_id = $1 
            AND scopes @> $2
        ",
        )
        .bind(self.guild_id.to_string())
        .bind(scopes)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get key: {e}"))?;

        let keys = data
            .iter()
            .map(|row| row.get::<String, _>("key"))
            .collect::<Vec<_>>();

        Ok(keys)
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct CliDataStoreProvider {
    pub guild_id: serenity::all::GuildId,
}

impl DataStoreProvider for CliDataStoreProvider {
    fn attempt_action(&self, _method: &str, _bucket: &str) -> Result<(), khronos_runtime::Error> {
        Ok(())
    }

    /// Returns a builtin data store given its name
    fn get_builtin_data_store(&self, name: &str) -> Option<Rc<dyn DataStoreImpl>> {
        if name == "CopyDataStore" {
            return Some(Rc::new(khronos_runtime::traits::ir::CopyDataStore {}));
        }

        None
    }

    /// Returns all public builtin data stores
    fn public_builtin_data_stores(&self) -> Vec<String> {
        vec!["CopyDataStore".to_string()] // TODO
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

    fn guild_id(&self) -> serenity::all::GuildId {
        self.guild_id
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
pub struct CliHttpServerProvider;

impl HTTPServerProvider for CliHttpServerProvider {
    fn attempt_action(&self, _bucket: &str, _path: String) -> Result<(), khronos_runtime::Error> {
        Ok(())
    }
}
