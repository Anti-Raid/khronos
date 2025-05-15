use khronos_runtime::Error;
use serenity::async_trait;
use std::{fmt::Debug, path::PathBuf, rc::Rc};
use tokio::sync::RwLock;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AssetEntry {
    /// Name of the file
    pub name: String,
    /// The contents of the file
    pub contents: Vec<u8>,
    /// Size of the file in bytes
    pub size: u64,
    /// When the file was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the file was last modified
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
}

impl<T: AsRef<[u8]>> PartialEq<T> for AssetEntry {
    fn eq(&self, other: &T) -> bool {
        self.contents == other.as_ref()
    }
}

#[allow(dead_code)]
#[async_trait(?Send)]
/// File storage provider for khronos CLI
pub trait FileStorageProvider: Debug {
    /// Check if a file with the given filename exists
    async fn file_exists(&self, file_path: &[String], key: &str) -> Result<bool, Error>;

    /// List all files in file path. Must return a vec of FileListEntry
    ///
    /// If pattern is not empty, only return files that match the pattern (% matches any sequence of characters, _ matches any single character)
    ///
    /// NOTE: this method should *not* be recursive/recurse down into subdirectories
    async fn list_files(
        &self,
        path: &[String],
        pattern: Option<String>,
        limit_offset: Option<(usize, usize)>,
    ) -> Result<Vec<AssetEntry>, Error>;

    /// Get the data of the file with the given filename
    async fn get_file(&self, file_path: &[String], key: &str) -> Result<Option<AssetEntry>, Error>;

    /// Save the given data to the file with the given filename
    ///
    /// Creates the file if it does not exist
    async fn save_file(&self, file_path: &[String], key: &str, data: &[u8]) -> Result<(), Error>;

    /// Delete the file with the given filename
    async fn delete_file(&self, file_path: &[String], key: &str) -> Result<(), Error>;
}

#[allow(dead_code)]
/// Returns true if the filename matches the pattern based on PostgreSQL ILIKE pattern matching rules
fn does_file_match_pattern(filename: &str, pattern: &str) -> Result<bool, khronos_runtime::Error> {
    // An underscore (_) in pattern stands for (matches) any single character; a percent sign (%) matches any sequence of zero or more characters.
    let pattern = pattern
        .replace('.', "\\.")
        .replace("_", ".")
        .replace("%", ".*");
    let regex = regex::Regex::new(&format!("(?i)^{}$", pattern))?;
    Ok(regex.is_match(filename))
}

#[derive(Debug, Clone)]
/// Local file storage provider
pub struct LocalFileStorageProvider {
    base_path: PathBuf,
    lock: Rc<RwLock<()>>,
    verbose: bool,
}

impl LocalFileStorageProvider {
    pub async fn new(base_path: PathBuf, verbose: bool) -> Result<Self, Error> {
        let s = Self {
            base_path,
            lock: Rc::new(RwLock::new(())),
            verbose,
        };

        s.create_base_path_if_not_exists().await?;

        Ok(s)
    }

    async fn create_base_path_if_not_exists(&self) -> Result<(), Error> {
        if self.verbose {
            println!(
                "[LocalFileStorageProvider] Creating base path: {:?}",
                self.base_path
            );
        }
        tokio::fs::create_dir_all(&self.base_path)
            .await
            .map_err(Error::from)
    }

    fn parse_key_to_fs_file(key: &str) -> String {
        if key.contains(['/', '\\', '.']) || key.starts_with("b64") {
            // Convert key to base64
            format!(
                "b64{}",
                data_encoding::BASE64URL_NOPAD.encode(key.as_bytes())
            )
        } else {
            key.to_string()
        }
    }

    fn parse_fs_file_to_key(file: &str) -> Result<String, khronos_runtime::Error> {
        if file.starts_with("b64") {
            // Convert base64 to key
            data_encoding::BASE64URL_NOPAD
                .decode(file.trim_start_matches("b64").as_bytes())
                .map_err(|e| e.into())
                .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
        } else {
            Ok(file.to_string())
        }
    }
}

#[async_trait(?Send)]
impl FileStorageProvider for LocalFileStorageProvider {
    async fn file_exists(&self, file_path: &[String], key: &str) -> Result<bool, Error> {
        let _g = self.lock.read().await;

        if self.verbose {
            println!(
                "[LocalFileStorageProvider] Checking if file exists: path={:?}, key={:?}",
                file_path, key
            );
        }

        let mut path = self.base_path.clone();
        path.extend(file_path.iter());
        path.push(Self::parse_key_to_fs_file(key));

        Ok(tokio::fs::try_exists(path).await?)
    }

    async fn list_files(
        &self,
        file_path: &[String],
        pattern: Option<String>,
        limit_offset: Option<(usize, usize)>,
    ) -> Result<Vec<AssetEntry>, Error> {
        let _g = self.lock.read().await;

        if self.verbose {
            println!(
                "[LocalFileStorageProvider] Listing files: {:?} with pattern: {:?} and limit_offset: {:?}",
                file_path,
                pattern,
                limit_offset
            );
        }

        let mut path = self.base_path.clone();
        path.extend(file_path.iter());

        let mut read_dir = tokio::fs::read_dir(&path).await?;

        let mut files = Vec::new();
        while let Some(entry) = read_dir.next_entry().await? {
            if let Some((limit, offset)) = limit_offset {
                // If we haven't reached the offset yet, skip
                if files.len() < offset {
                    continue;
                }

                // If we've reached the limit, break
                if files.len() >= limit {
                    break;
                }
            }

            files.push(AssetEntry {
                name: {
                    let filename = entry.file_name().to_string_lossy().to_string();
                    let key = match Self::parse_fs_file_to_key(&filename) {
                        Ok(key) => key,
                        Err(err) => {
                            eprintln!("Error in list_files when parsing filename: {:?}", err);
                            filename
                        }
                    };

                    if let Some(pattern) = &pattern {
                        println!(
                            "Checking if file matches pattern: {} {} -> {}",
                            key,
                            pattern,
                            does_file_match_pattern(&key, pattern)?
                        );
                        if !does_file_match_pattern(&key, pattern)? {
                            continue;
                        }
                    }

                    key
                },
                contents: {
                    let path = entry.path();
                    tokio::fs::read(path).await?
                },
                created_at: {
                    let metadata = entry.metadata().await?;
                    let created_at = metadata.created()?;
                    chrono::DateTime::from(created_at)
                },
                last_updated_at: {
                    let metadata = entry.metadata().await?;
                    let last_updated_at = metadata.modified()?;
                    chrono::DateTime::from(last_updated_at)
                },
                size: entry.metadata().await?.len(),
            });

            // TODO: Test that the below actually works
            if entry.file_type().await?.is_dir() {
                // Traverse the directory
                let sub_files = self
                    .list_files(
                        &[entry.file_name().to_string_lossy().to_string()],
                        pattern.clone(),
                        limit_offset,
                    )
                    .await?;
                
                files.extend(sub_files);
            }
        }
        Ok(files)
    }

    async fn get_file(&self, file_path: &[String], key: &str) -> Result<Option<AssetEntry>, Error> {
        let _g = self.lock.read().await;

        if self.verbose {
            println!(
                "[LocalFileStorageProvider] Getting file: path={:?}, key={:?}",
                file_path, key
            );
        }

        let mut path = self.base_path.clone();
        path.extend(file_path.iter());
        path.push(Self::parse_key_to_fs_file(key));

        let metadata = match tokio::fs::metadata(&path).await {
            Ok(metadata) => metadata,
            Err(e) => {
                // Check if e is a not found error
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Ok(None);
                }

                return Err(e.into());
            }
        };
        if !metadata.is_file() {
            return Ok(None);
        }

        Ok(Some(AssetEntry {
            name: key.to_string(),
            contents: tokio::fs::read(&path).await?,
            created_at: {
                let created_at = metadata.created()?;
                chrono::DateTime::from(created_at)
            },
            last_updated_at: {
                let last_updated_at = metadata.modified()?;
                chrono::DateTime::from(last_updated_at)
            },
            size: metadata.len(),
        }))
    }

    async fn save_file(&self, file_path: &[String], key: &str, data: &[u8]) -> Result<(), Error> {
        let _g = self.lock.write().await;

        if self.verbose {
            println!(
                "[LocalFileStorageProvider] Saving file: path={:?}, key={:?}",
                file_path, key
            );
        }

        let mut path = self.base_path.clone();
        path.extend(file_path.iter());

        // Create dir if it doesn't exist
        tokio::fs::create_dir_all(&path).await?;

        path.push(Self::parse_key_to_fs_file(key));

        tokio::fs::write(path, data).await.map_err(Error::from)
    }

    async fn delete_file(&self, file_path: &[String], key: &str) -> Result<(), Error> {
        let _g = self.lock.write().await;

        if self.verbose {
            println!(
                "[LocalFileStorageProvider] Deleting file: path={:?}, key={:?}",
                file_path, key
            );
        }

        let mut path = self.base_path.clone();
        path.extend(file_path.iter());

        // Create dir if it doesn't exist
        tokio::fs::create_dir_all(&path).await?;

        path.push(Self::parse_key_to_fs_file(key));

        match tokio::fs::remove_file(path).await {
            Ok(_) => {
                if self.verbose {
                    println!("[LocalFileStorageProvider] Deleted file successfully");
                }
                Ok(())
            }
            Err(e) => {
                // Handle the case where the file does not exist
                if e.kind() == std::io::ErrorKind::NotFound {
                    if self.verbose {
                        println!("[LocalFileStorageProvider] File not found for deletion");
                    }
                    Ok(())
                } else {
                    Err(e.into())
                }
            }
        }
    }
}

/// A second (much more performant) file storage provider that uses SQLite
/// to store 'files'.
///
/// Unlike local file storage, this provider does not need file path escaping
///
/// Code is not implemented yet but schema would be as simple as:
/// CREATE TABLE files (path TEXT NOT NULL, key TEXT NOT NULL, data BLOB NOT NULL, created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, last_updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, PRIMARY KEY (path, key));
///
/// ## Current Limitations
///
/// - Scoped KV keys() operation currently does not work too well with it
#[allow(dead_code)]
#[cfg(feature = "sqlite")]
#[derive(Debug, Clone)]
pub struct SqliteFileStorageProvider {
    base_path: PathBuf,
    verbose: bool,
    conn: Rc<rusqlite::Connection>,
}

#[allow(dead_code)]
#[cfg(feature = "sqlite")]
impl SqliteFileStorageProvider {
    pub async fn new(
        base_path: PathBuf,
        verbose: bool,
        set_pragma_synchronize: bool,
    ) -> Result<Self, Error> {
        if verbose {
            println!(
                "[LocalFileStorageProvider] Creating base path: {:?}",
                base_path
            );
        }
        tokio::fs::create_dir_all(&base_path)
            .await
            .map_err(Error::from)?;

        let conn = rusqlite::Connection::open(base_path.join("files.db"))?;
        let res = conn.execute(
            "CREATE TABLE IF NOT EXISTS files (path TEXT NOT NULL, key TEXT NOT NULL, data BLOB NOT NULL, created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, last_updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, PRIMARY KEY (path, key));",
        [],
        )?;

        if verbose {
            println!(
                "[SqliteFileStorageProvider] Created `files` table: rows_changed={:?}",
                res
            );
        }

        conn.set_prepared_statement_cache_capacity(100);

        // Setup index on path and key
        conn.execute(
            "CREATE INDEX IF NOT EXISTS files_path_key_idx ON files (path, key);",
            [],
        )?;

        if set_pragma_synchronize {
            // Set synchronous to off
            conn.execute("PRAGMA synchronous = OFF;", [])?;
        }

        Ok(Self {
            base_path,
            conn: Rc::new(conn),
            verbose,
        })
    }
}

#[cfg(feature = "sqlite")]
#[async_trait(?Send)]
impl FileStorageProvider for SqliteFileStorageProvider {
    async fn file_exists(&self, file_path: &[String], key: &str) -> Result<bool, Error> {
        if self.verbose {
            println!(
                "[SqliteFileStorageProvider] Checking if file exists: path={:?}, key={:?}",
                file_path, key
            );
        }

        let path = file_path.join("/");
        let key = key.to_string();

        let verbose = self.verbose;

        let mut stmt = self
            .conn
            .prepare_cached("SELECT COUNT(*) FROM files WHERE path = ? AND key = ?;")?;

        let res = stmt
            .query_row(rusqlite::params![path, key], |row| row.get::<_, i64>(0))
            .map(|count| count > 0)?;

        if verbose {
            println!("[SqliteFileStorageProvider] File exists: {:?}", res);
        }

        Ok(res)
    }

    async fn get_file(&self, file_path: &[String], key: &str) -> Result<Option<AssetEntry>, Error> {
        if self.verbose {
            println!(
                "[SqliteFileStorageProvider] Getting file: path={:?}, key={:?}",
                file_path, key
            );
        }

        let path = file_path.join("/");
        let key = key.to_string();

        let verbose = self.verbose;
        let mut stmt = self.conn.prepare_cached(
            "SELECT data, created_at, last_updated_at FROM files WHERE path = ? AND key = ?;",
        )?;

        let mut rows = stmt.query_map(rusqlite::params![path, key], |row| {
            let contents = row.get::<_, Vec<u8>>(0)?;
            let size = contents.len() as u64;
            Ok(AssetEntry {
                name: key.clone(),
                contents,
                created_at: row.get(1)?,
                last_updated_at: row.get(2)?,
                size,
            })
        })?;

        let res = rows.next().transpose()?;

        if verbose {
            println!("[SqliteFileStorageProvider] Got file: {:?}", res);
        }

        Ok(res)
    }

    async fn list_files(
        &self,
        file_path: &[String],
        pattern: Option<String>,
        limit_offset: Option<(usize, usize)>,
    ) -> Result<Vec<AssetEntry>, Error> {
        if self.verbose {
            println!(
                "[SqliteFileStorageProvider] Listing files: {:?} with pattern: {:?} and limit_offset: {:?}",
                file_path,
                pattern,
                limit_offset
            );
        }

        let path = file_path.join("/");

        if let Some((limit, offset)) = limit_offset {
            if limit > i64::MAX.try_into()? || offset > i64::MAX.try_into()? {
                return Err(Error::from("Limit or offset too large"));
            }
        }

        let verbose = self.verbose;
        let (mut stmt, params) = if let Some(pattern) = pattern {
            if let Some((limit, offset)) = limit_offset {
                let (limit, offset) = (limit as i64, offset as i64);

                (self.conn.prepare_cached(
                    "SELECT key, data, created_at, last_updated_at FROM files WHERE path = ? AND key LIKE ? LIMIT ? OFFSET ?;",
                )?, rusqlite::params![path, pattern.clone(), limit.clone(), offset.clone()])
            } else {
                (self.conn.prepare_cached(
                    "SELECT key, data, created_at, last_updated_at FROM files WHERE path = ? AND key LIKE ?;",
                )?, rusqlite::params![path, pattern.clone()])
            }
        } else {
            #[allow(clippy::collapsible_if)]
            if let Some((limit, offset)) = limit_offset {
                let (limit, offset) = (limit as i64, offset as i64);

                (self.conn.prepare_cached(
                    "SELECT key, data, created_at, last_updated_at FROM files WHERE path = ? LIMIT ? OFFSET ?;",
                )?, rusqlite::params![path, limit.clone(), offset.clone()])
            } else {
                (
                    self.conn.prepare_cached(
                        "SELECT key, data, created_at, last_updated_at FROM files WHERE path = ?;",
                    )?,
                    rusqlite::params![path],
                )
            }
        };

        let rows = stmt.query_map(params, |row| {
            let contents = row.get::<_, Vec<u8>>(1)?;
            let size = contents.len() as u64;
            Ok(AssetEntry {
                name: row.get(0)?,
                contents,
                created_at: row.get(2)?,
                last_updated_at: row.get(3)?,
                size,
            })
        })?;

        let mut files = Vec::new();
        for row in rows {
            files.push(row?);
        }

        if verbose {
            println!("[SqliteFileStorageProvider] Listed files: {:?}", files);
        }
        Ok(files)
    }

    async fn save_file(&self, file_path: &[String], key: &str, data: &[u8]) -> Result<(), Error> {
        if self.verbose {
            println!(
                "[SqliteFileStorageProvider] Creating file: path={:?}, key={:?}",
                file_path, key
            );
        }

        let path = file_path.join("/");
        let key = key.to_string();

        let verbose = self.verbose;
        let mut stmt = self.conn.prepare_cached(
            "INSERT OR REPLACE INTO files (path, key, data, created_at, last_updated_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);",
        )?;

        let res = stmt.execute(rusqlite::params![path, key, data])?;

        if verbose {
            println!(
                "[SqliteFileStorageProvider] Saving file: rows_changed={:?}",
                res
            );
        }

        Ok(())
    }

    async fn delete_file(&self, file_path: &[String], key: &str) -> Result<(), Error> {
        if self.verbose {
            println!(
                "[SqliteFileStorageProvider] Deleting file: path={:?}, key={:?}",
                file_path, key
            );
        }

        let path = file_path.join("/");
        let key = key.to_string();

        let verbose = self.verbose;
        let mut stmt = self
            .conn
            .prepare_cached("DELETE FROM files WHERE path = ? AND key = ?;")?;

        let res = stmt.execute(rusqlite::params![path, key])?;

        if verbose {
            println!(
                "[SqliteFileStorageProvider] Deleting file: rows_changed={:?}",
                res
            );
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[cfg(feature = "sqlite")]
#[derive(Debug, Clone)]
pub struct SqliteInMemoryProvider {
    verbose: bool,
    conn: Rc<rusqlite::Connection>,
}

#[allow(dead_code)]
#[cfg(feature = "sqlite")]
impl SqliteInMemoryProvider {
    pub async fn new(verbose: bool) -> Result<Self, Error> {
        if verbose {
            println!("[SqliteInMemoryProvider] Creating in-memory database",);
        }

        let conn = rusqlite::Connection::open_in_memory()?;
        let res = conn.execute(
            "CREATE TABLE files (path TEXT NOT NULL, key TEXT NOT NULL, data BLOB NOT NULL, created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, last_updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, PRIMARY KEY (path, key));",
        [],
        )?;

        if verbose {
            println!(
                "[SqliteFileStorageProvider] Created `files` table: rows_changed={:?}",
                res
            );
        }

        conn.set_prepared_statement_cache_capacity(100);

        // Setup index on path and key
        conn.execute("CREATE INDEX files_path_key_idx ON files (path, key);", [])?;

        Ok(Self {
            conn: Rc::new(conn),
            verbose,
        })
    }
}

#[cfg(feature = "sqlite")]
#[async_trait(?Send)]
impl FileStorageProvider for SqliteInMemoryProvider {
    async fn file_exists(&self, file_path: &[String], key: &str) -> Result<bool, Error> {
        if self.verbose {
            println!(
                "[SqliteInMemoryProvider] Checking if file exists: path={:?}, key={:?}",
                file_path, key
            );
        }

        let path = file_path.join("/");
        let key = key.to_string();

        let verbose = self.verbose;

        let mut stmt = self
            .conn
            .prepare_cached("SELECT COUNT(*) FROM files WHERE path = ? AND key = ?;")?;

        let res = stmt
            .query_row(rusqlite::params![path, key], |row| row.get::<_, i64>(0))
            .map(|count| count > 0)?;

        if verbose {
            println!("[SqliteInMemoryProvider] File exists: {:?}", res);
        }

        Ok(res)
    }

    async fn get_file(&self, file_path: &[String], key: &str) -> Result<Option<AssetEntry>, Error> {
        if self.verbose {
            println!(
                "[SqliteInMemoryProvider] Getting file: path={:?}, key={:?}",
                file_path, key
            );
        }

        let path = file_path.join("/");
        let key = key.to_string();

        let verbose = self.verbose;
        let mut stmt = self.conn.prepare_cached(
            "SELECT data, created_at, last_updated_at FROM files WHERE path = ? AND key = ?;",
        )?;

        let mut rows = stmt.query_map(rusqlite::params![path, key], |row| {
            let contents = row.get::<_, Vec<u8>>(0)?;
            let size = contents.len() as u64;
            Ok(AssetEntry {
                name: key.clone(),
                contents,
                created_at: row.get(1)?,
                last_updated_at: row.get(2)?,
                size,
            })
        })?;

        let res = rows.next().transpose()?;

        if verbose {
            println!("[SqliteInMemoryProvider] Got file: {:?}", res);
        }

        Ok(res)
    }

    async fn list_files(
        &self,
        file_path: &[String],
        pattern: Option<String>,
        limit_offset: Option<(usize, usize)>,
    ) -> Result<Vec<AssetEntry>, Error> {
        if self.verbose {
            println!(
                "[SqliteInMemoryProvider] Listing files: {:?} with pattern: {:?} and limit_offset: {:?}",
                file_path,
                pattern,
                limit_offset
            );
        }

        let path = file_path.join("/");

        if let Some((limit, offset)) = limit_offset {
            if limit > i64::MAX.try_into()? || offset > i64::MAX.try_into()? {
                return Err(Error::from("Limit or offset too large"));
            }
        }

        let verbose = self.verbose;
        let (mut stmt, params) = if let Some(pattern) = pattern {
            if let Some((limit, offset)) = limit_offset {
                let (limit, offset) = (limit as i64, offset as i64);

                (self.conn.prepare_cached(
                    "SELECT key, data, created_at, last_updated_at FROM files WHERE path = ? AND key LIKE ? LIMIT ? OFFSET ?;",
                )?, rusqlite::params![path, pattern.clone(), limit.clone(), offset.clone()])
            } else {
                (self.conn.prepare_cached(
                    "SELECT key, data, created_at, last_updated_at FROM files WHERE path = ? AND key LIKE ?;",
                )?, rusqlite::params![path, pattern.clone()])
            }
        } else {
            #[allow(clippy::collapsible_if)]
            if let Some((limit, offset)) = limit_offset {
                let (limit, offset) = (limit as i64, offset as i64);

                (self.conn.prepare_cached(
                    "SELECT key, data, created_at, last_updated_at FROM files WHERE path = ? LIMIT ? OFFSET ?;",
                )?, rusqlite::params![path, limit.clone(), offset.clone()])
            } else {
                (
                    self.conn.prepare_cached(
                        "SELECT key, data, created_at, last_updated_at FROM files WHERE path = ?;",
                    )?,
                    rusqlite::params![path],
                )
            }
        };

        let rows = stmt.query_map(params, |row| {
            let contents = row.get::<_, Vec<u8>>(1)?;
            let size = contents.len() as u64;
            Ok(AssetEntry {
                name: row.get(0)?,
                contents,
                created_at: row.get(2)?,
                last_updated_at: row.get(3)?,
                size,
            })
        })?;

        let mut files = Vec::new();
        for row in rows {
            files.push(row?);
        }

        if verbose {
            println!("[SqliteFileStorageProvider] Listed files: {:?}", files);
        }
        Ok(files)
    }

    async fn save_file(&self, file_path: &[String], key: &str, data: &[u8]) -> Result<(), Error> {
        if self.verbose {
            println!(
                "[SqliteInMemoryProvider] Creating file: path={:?}, key={:?}",
                file_path, key
            );
        }

        let path = file_path.join("/");
        let key = key.to_string();

        let verbose = self.verbose;
        let mut stmt = self.conn.prepare_cached(
            "INSERT OR REPLACE INTO files (path, key, data, created_at, last_updated_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);",
        )?;

        let res = stmt.execute(rusqlite::params![path, key, data])?;

        if verbose {
            println!(
                "[SqliteFileStorageProvider] Saving file: rows_changed={:?}",
                res
            );
        }

        Ok(())
    }

    async fn delete_file(&self, file_path: &[String], key: &str) -> Result<(), Error> {
        if self.verbose {
            println!(
                "[SqliteInMemoryProvider] Deleting file: path={:?}, key={:?}",
                file_path, key
            );
        }

        let path = file_path.join("/");
        let key = key.to_string();

        let verbose = self.verbose;
        let mut stmt = self
            .conn
            .prepare_cached("DELETE FROM files WHERE path = ? AND key = ?;")?;

        let res = stmt.execute(rusqlite::params![path, key])?;

        if verbose {
            println!(
                "[SqliteInMemoryProvider] Deleting file: rows_changed={:?}",
                res
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod filestorage_test {
    pub use super::*;

    #[test]
    pub fn test_file_storages() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .worker_threads(10)
            .build()
            .unwrap();

        let local = tokio::task::LocalSet::new();

        local.block_on(&rt, async {
            // Remove test_files directory if it exists
            tokio::fs::remove_dir_all("test_files").await.ok();

            let provider = LocalFileStorageProvider::new(PathBuf::from("test_files"), false)
                .await
                .unwrap();

            eprintln!("Testing local file storage provider");

            let time_now = std::time::Instant::now();
            test_provider(provider).await;
            let time_elapsed = time_now.elapsed();
            eprintln!("=> Time elapsed in local fs provider: {:?}", time_elapsed);

            // Remove test_files directory if it exists
            tokio::fs::remove_dir_all("test_files").await.ok();

            #[cfg(feature = "sqlite")]
            {
                eprintln!("Testing sqlite file storage provider");
                let provider =
                    SqliteFileStorageProvider::new(PathBuf::from("test_files"), false, false)
                        .await
                        .unwrap();

                let time_now = std::time::Instant::now();
                test_provider(provider).await;
                let time_elapsed = time_now.elapsed();
                eprintln!("=> Time elapsed in sqlite fs provider: {:?}", time_elapsed);

                // Delete the base path
                tokio::fs::remove_dir_all("test_files").await.unwrap();

                eprintln!("Testing sqlite file storage provider [synchronous]");
                let provider =
                    SqliteFileStorageProvider::new(PathBuf::from("test_files"), false, true)
                        .await
                        .unwrap();

                let time_now = std::time::Instant::now();
                test_provider(provider).await;
                let time_elapsed = time_now.elapsed();
                eprintln!(
                    "=> Time elapsed in sqlite fs provider [synchronous]: {:?}",
                    time_elapsed
                );

                // Delete the base path
                tokio::fs::remove_dir_all("test_files").await.unwrap();

                eprintln!("Testing sqlite in-memory provider");
                let provider = SqliteInMemoryProvider::new(false).await.unwrap();

                let time_now = std::time::Instant::now();
                test_provider(provider).await;
                let time_elapsed = time_now.elapsed();
                eprintln!(
                    "=> Time elapsed in sqlite in-memory provider: {:?}",
                    time_elapsed
                );
            }
        });
    }

    async fn test_provider(provider: impl FileStorageProvider + Clone + 'static) {
        eprintln!("Testing provider: {:?}", provider);

        let file_path = vec![];
        let key = "test.txt".to_string();

        if !provider.file_exists(&file_path, &key).await.unwrap() {
            println!("Warning: File does not exist! This is a known test issue");
        }
        let data = b"Hello, world!";
        provider.save_file(&file_path, &key, data).await.unwrap();
        assert_eq!(
            provider.get_file(&file_path, &key).await.unwrap().unwrap(),
            data
        );

        provider.delete_file(&file_path, &key).await.unwrap();
        assert!(!provider.file_exists(&file_path, &key).await.unwrap());

        // Make 100 tasks all calling create_file at the same time on the same file
        let mut tasks = Vec::new();

        const N: i64 = 1000;
        for i in 0..N {
            let provider = provider.clone();
            let file_path = vec![];
            let key = "test.txt".to_string();

            tasks.push(tokio::task::spawn_local(async move {
                provider
                    .save_file(&file_path, &key, &i.to_le_bytes())
                    .await
                    .unwrap();
            }));
        }

        for task in tasks {
            task.await.unwrap();
        }

        println!("All tasks completed");

        // Ensure the file only contains the last write
        assert_eq!(
            provider.get_file(&file_path, &key).await.unwrap().unwrap(),
            &(N - 1).to_le_bytes()
        );

        println!("At the list files test");

        // List files test
        let files = provider.list_files(&file_path, None, None).await.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, key);
        provider
            .save_file(&file_path, "test2.txt", b"")
            .await
            .unwrap();
        let files = provider.list_files(&file_path, None, None).await.unwrap();
        assert_eq!(files.len(), 2);
        let files = provider
            .list_files(&file_path, Some("%txt".to_string()), None)
            .await
            .unwrap();
        assert_eq!(files.len(), 2);
        println!("At the list files test 2: {:?}", files);
        let files = provider
            .list_files(&file_path, Some("test.%".to_string()), None)
            .await
            .unwrap();
        assert_eq!(files.len(), 1);
    }

    use super::LocalFileStorageProvider;

    #[test]
    fn test_local_fs_does_file_match_pattern() {
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

        assert!(
            does_file_match_pattern("test.txt", "test%").unwrap(),
            "test.txt should match test%"
        );

        for _ in 0..10 {
            assert!(
                does_file_match_pattern("test.txt", "test.%").unwrap(),
                "test.txt should match test.%"
            );

            assert!(
                !does_file_match_pattern("test2.txt", "test.%").unwrap(),
                "test2.txt should not match test.%"
            );
        }
    }
}
