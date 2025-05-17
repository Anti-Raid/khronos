use crate::plugins::antiraid::promise::LuaPromise;
use crate::primitives::create_userdata_iterator_with_fields;
use crate::traits::context::KhronosContext;
use crate::traits::objectstorageprovider::ObjectStorageProvider;
use crate::utils::executorscope::ExecutorScope;
use crate::TemplateContextRef;
use crate::plugins::antiraid::promise::UserDataLuaPromise;
use mlua::prelude::*;
use crate::utils::khronos_value::KhronosValue;
use crate::to_struct;
use serde::{Serialize, Deserialize};
use mlua::Buffer;

to_struct! {
    pub struct ObjectMetadata {
        pub key: String,
        pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
        pub size: i64,
        pub etag: Option<String>,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectStorageReadRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct DownloadFileOpts {
    pub range: Option<ObjectStorageReadRange>,
}

/// Represents a path to an object in the object storage
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ObjectStoragePath {
    pub path: String,
}

impl ObjectStoragePath {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl LuaUserData for ObjectStoragePath {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("path", |_, this| Ok(this.path.clone()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(this.path.clone())
        });
    }
}

#[derive(Clone)]
pub struct Bucket<T: KhronosContext> {
    context: T,
    objectstorage_provider: T::ObjectStorageProvider,
}

impl<T: KhronosContext> Bucket<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.context.has_cap(&format!("objectstorage:{}", action)) {
            return Err(LuaError::runtime(format!(
                "Objectstorage action `{}` not allowed in this template context",
                action
            )));
        }

        self.objectstorage_provider
            .attempt_action(&action)
            .map_err(|e| LuaError::external(e.to_string()))?;

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for Bucket<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("bucket_name", |_, this| Ok(this.objectstorage_provider.bucket_name()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
/*
    /// List all files in the servers bucket with the specified (optional) prefix.
    async fn list_files(&self, prefix: Option<String>) -> Result<Vec<ListObjectsResponse>, crate::Error>;

    /// Returns if a specific key exists in the key-value store.
    async fn file_exists(&self, key: String) -> Result<bool, crate::Error>;

    /// Downloads a file from the key-value store.
    async fn download_file(&self, key: String) -> Result<Vec<u8>, crate::Error>;

    /// Returns the URL to a file in the key-value store.
    async fn get_file_url(&self, key: String) -> Result<String, crate::Error>;

    /// Upload a file to the key-value store.
    async fn upload_file(&self, key: String, data: Vec<u8>) -> Result<(), crate::Error>;

    /// Delete a file from the key-value store.
    async fn delete_file(&self, key: String) -> Result<(), crate::Error>;
*/

        methods.add_promise_method("list_files", async move |_, this, prefix: Option<String>| {
            this.check_action("list_files".to_string())?;

            let result = this.objectstorage_provider.list_files(prefix).await
            .map_err(|e| {
                LuaError::external(e.to_string())
            })?;
            
            let kv: KhronosValue = result
            .into_iter()
            .map(|r| {
                ObjectMetadata {
                    key: r.key,
                    last_modified: r.last_modified,
                    size: r.size,
                    etag: r.etag,
                }
            })
            .collect::<Vec<_>>().into();

            Ok(kv)
        });

        methods.add_promise_method("file_exists", async move |_, this, key: String| {
            this.check_action("file_exists".to_string())?;

            let result = this.objectstorage_provider.file_exists(key).await
            .map_err(|e| {
                LuaError::external(e.to_string())
            })?;

            Ok(result)
        });

        methods.add_promise_method("download_file", async move |lua, this, (key, opts): (String, LuaValue)| {
            this.check_action("download_file".to_string())?;

            let opts = lua.from_value::<Option<DownloadFileOpts>>(opts)
                .map_err(|e| {
                    LuaError::external(e.to_string())
                })?
                .unwrap_or_default();

            // TODO: Support range at object storage level

            let result = this.objectstorage_provider.download_file(key).await
            .map_err(|e| {
                LuaError::external(e.to_string())
            })?;

            let len = if let Some(ref range) = opts.range {
                range.end - range.start
            } else {
                result.len()
            };

            if let Some(memory_limit) = this.context.memory_limit() {
                if memory_limit > lua.used_memory() && memory_limit - lua.used_memory() < len {
                    return Err(LuaError::external(format!(
                        "File size {} exceeds available memory ({} bytes / {} total bytes)",
                        len,
                        memory_limit - lua.used_memory(),
                        memory_limit
                    )));
                }
            }

            let buffer = if let Some(range) = opts.range {
                lua.create_buffer(&result[range.start..range.end])?
            } else {
                lua.create_buffer(&result)? 
            };

            Ok(buffer)
        });

        methods.add_promise_method("get_file_url", async move |_, this, key: String| {
            this.check_action("get_file_url".to_string())?;

            let result = this.objectstorage_provider.get_file_url(key).await
            .map_err(|e| {
                LuaError::external(e.to_string())
            })?;

            Ok(result)
        });

        methods.add_promise_method("upload_file", async move |_, this, (key, data): (String, Buffer)| {
            this.check_action("upload_file".to_string())?;

            this.objectstorage_provider.upload_file(key, data.to_vec()).await
            .map_err(|e| {
                LuaError::external(e.to_string())
            })?;

            Ok(())
        });

        methods.add_promise_method("delete_file", async move |_, this, key: String| {
            this.check_action("delete_file".to_string())?;

            this.objectstorage_provider.delete_file(key).await
            .map_err(|e| {
                LuaError::external(e.to_string())
            })?;

            Ok(())
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<Bucket<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "bucket_name",
                    // Methods
                    "list_files",
                    "file_exists",
                    "download_file",
                    "get_file_url",
                    "upload_file",
                    "delete_file",
                ],
            )
        });
    }
}

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    // Create a new bucket handle
    module.set(
        "new",
        lua.create_function(
            |lua, (token, scope): (TemplateContextRef<T>, Option<String>)| {
                let scope = ExecutorScope::scope_str(scope)?;
                let Some(objectstorage_provider) = token.context.objectstorage_provider(scope) else {
                    return Err(LuaError::external(
                        "The datastore plugin is not supported in this context",
                    ));
                };

                let bucket = Bucket {
                    context: token.context.clone(),
                    objectstorage_provider,
                }
                .into_lua(lua)?;
                Ok(bucket)
            },
        )?,
    )?;

    // Create a new object storage path
    module.set(
        "ObjectStoragePath",
        lua.create_function(|_, path: String| {
            let object_storage_path = ObjectStoragePath::new(path);
            Ok(object_storage_path)
        })?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
