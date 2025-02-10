use khronos_runtime::Error;
use serenity::async_trait;
use std::{fmt::Debug, path::PathBuf, rc::Rc};
use tokio::sync::RwLock;

#[derive(Debug)]
#[allow(dead_code)]
pub struct FileListEntry {
    /// Name of the file
    pub name: String,
    /// 0 if file, 1 if directory
    pub file_typ: u8,
    /// Size of the file in bytes
    pub size: u64,
}

#[allow(dead_code)]
#[async_trait(?Send)]
/// File storage provider for khronos CLI
pub trait FileStorageProvider: Debug {
    /// Create a file with the given filename
    async fn create_file(&self, file_path: &[String]) -> Result<(), Error>;

    /// Check if a file with the given filename exists
    async fn file_exists(&self, file_path: &[String]) -> Result<bool, Error>;

    /// List all files in file path. Must return a vec of FileListEntry
    async fn list_files(&self, path: &[String]) -> Result<Vec<FileListEntry>, Error>;

    /// Get the data of the file with the given filename
    async fn get_file(&self, file_path: &[String]) -> Result<Vec<u8>, Error>;

    /// Save the given data to the file with the given filename
    ///
    /// Creates the file if it does not exist
    async fn save_file(&self, file_path: &[String], data: &[u8]) -> Result<(), Error>;

    /// Delete the file with the given filename
    async fn delete_file(&self, file_path: &[String]) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
/// Local file storage provider
pub struct LocalFileStorageProvider {
    base_path: PathBuf,
    lock: Rc<RwLock<()>>,
}

impl LocalFileStorageProvider {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            lock: Rc::new(RwLock::new(())),
        }
    }

    fn create_base_path_if_not_exists(&self) -> Result<(), Error> {
        std::fs::create_dir_all(&self.base_path).map_err(Error::from)
    }
}

#[async_trait(?Send)]
impl FileStorageProvider for LocalFileStorageProvider {
    async fn create_file(&self, file_path: &[String]) -> Result<(), Error> {
        let _g = self.lock.write().await;
        self.create_base_path_if_not_exists()?;
        let mut path = self.base_path.clone();
        path.extend(file_path.iter());
        tokio::fs::File::create(path).await?;
        Ok(())
    }

    async fn file_exists(&self, file_path: &[String]) -> Result<bool, Error> {
        let _g = self.lock.write().await;
        self.create_base_path_if_not_exists()?;
        let mut path = self.base_path.clone();
        path.extend(file_path.iter());
        Ok(tokio::fs::try_exists(path).await?)
    }

    async fn list_files(&self, file_path: &[String]) -> Result<Vec<FileListEntry>, Error> {
        let _g = self.lock.write().await;
        self.create_base_path_if_not_exists()?;
        let mut path = self.base_path.clone();
        path.extend(file_path.iter());

        let mut files = Vec::new();
        while let Some(entry) = tokio::fs::read_dir(&path).await?.next_entry().await? {
            files.push(FileListEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                file_typ: {
                    let file_type = entry.file_type().await?;
                    if file_type.is_dir() {
                        1
                    } else {
                        0
                    }
                },
                size: entry.metadata().await?.len(),
            });
        }
        Ok(files)
    }

    async fn get_file(&self, file_path: &[String]) -> Result<Vec<u8>, Error> {
        let _g = self.lock.write().await;
        self.create_base_path_if_not_exists()?;
        let mut path = self.base_path.clone();
        path.extend(file_path.iter());
        tokio::fs::read(path).await.map_err(Error::from)
    }

    async fn save_file(&self, file_path: &[String], data: &[u8]) -> Result<(), Error> {
        let _g = self.lock.write().await;
        self.create_base_path_if_not_exists()?;
        let mut path = self.base_path.clone();
        path.extend(file_path.iter());
        tokio::fs::write(path, data).await.map_err(Error::from)
    }

    async fn delete_file(&self, file_path: &[String]) -> Result<(), Error> {
        let _g = self.lock.write().await;
        self.create_base_path_if_not_exists()?;
        let mut path = self.base_path.clone();
        path.extend(file_path.iter());
        tokio::fs::remove_file(path).await.map_err(Error::from)
    }
}

#[cfg(test)]
mod filestorage_test {
    pub use super::*;

    #[test]
    pub fn test_local_file_storage() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .worker_threads(10)
            .build()
            .unwrap();

        let local = tokio::task::LocalSet::new();

        local.block_on(&rt, async {
            let provider = LocalFileStorageProvider::new(PathBuf::from("test_files"));
            let file_path = vec!["test.txt".to_string()];

            assert!(!provider.file_exists(&file_path).await.unwrap());
            let data = b"Hello, world!";
            provider.save_file(&file_path, data).await.unwrap();
            assert_eq!(provider.get_file(&file_path).await.unwrap(), data);

            provider.delete_file(&file_path).await.unwrap();
            assert!(!provider.file_exists(&file_path).await.unwrap());

            // Make 100 tasks all calling create_file at the same time on the same file
            let mut tasks = Vec::new();

            for i in 0..100 {
                let provider = provider.clone();
                let file_path = vec!["test.txt".to_string()];

                tasks.push(tokio::task::spawn_local(async move {
                    provider.save_file(&file_path, &[i]).await.unwrap();
                }));
            }

            for task in tasks {
                task.await.unwrap();
            }

            // Ensure the file only contains the last write
            assert_eq!(provider.get_file(&file_path).await.unwrap(), &[99]);

            // Delete the base path
            tokio::fs::remove_dir_all("test_files").await.unwrap();
        });
    }
}
