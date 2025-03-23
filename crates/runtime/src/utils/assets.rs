use std::{borrow::Cow, path::PathBuf, rc::Rc};

/// An asset manager is responsible for loading read-only assets.
///
/// This can/will be used in AntiRaid (at least) for multifile scripts
pub trait AssetManager {
    /// Gets the file contents given normalized path
    fn get_file(&self, path: &str) -> Result<Cow<'_, str>, crate::Error>;
}

/// All Into<AssetManager> implementations should be able to be used as an AssetManager
impl<T: ?Sized> AssetManager for &T
where
    T: AssetManager,
{
    fn get_file(&self, path: &str) -> Result<Cow<'_, str>, crate::Error> {
        (**self).get_file(path)
    }
}

impl<T: AssetManager> AssetManager for Box<T> {
    fn get_file(&self, path: &str) -> Result<Cow<'_, str>, crate::Error> {
        (**self).get_file(path)
    }
}

impl<T: AssetManager> AssetManager for Rc<T> {
    fn get_file(&self, path: &str) -> Result<Cow<'_, str>, crate::Error> {
        (**self).get_file(path)
    }
}

/// A simple fs-based asset manager for testing purposes
pub struct FileAssetManager {
    root_path: PathBuf,
}

impl FileAssetManager {
    /// Creates a new `FileAssetManager` with the given root path.
    ///
    /// # Arguments
    ///
    /// * `root_path` - A string slice that holds the root path for the asset manager.
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }
}

impl AssetManager for FileAssetManager {
    fn get_file(&self, path: &str) -> Result<Cow<'_, str>, crate::Error> {
        log::debug!("Current dir: {:?}", std::env::current_dir());

        let path = self.root_path.join(path);
        log::debug!("[AssetFS] Getting file: {}", path.display());
        if let Ok(file) = std::fs::read_to_string(&path) {
            return Ok(Cow::Owned(file));
        }

        Err(format!("File not found: {}", path.display()).into())
    }
}

/// A simple in-memory asset manager for testing purposes
pub struct HashMapAssetManager {
    assets: std::collections::HashMap<String, String>,
}

impl HashMapAssetManager {
    /// Creates a new `HashMapAssetManager` with the given assets.
    ///
    /// # Arguments
    ///
    /// * `assets` - A HashMap that holds the assets for the asset manager.
    pub fn new(assets: std::collections::HashMap<String, String>) -> Self {
        Self { assets }
    }
}

impl AssetManager for HashMapAssetManager {
    fn get_file(&self, path: &str) -> Result<Cow<'_, str>, crate::Error> {
        log::debug!("[Require] Getting file: {}", path);
        if let Some(file) = self.assets.get(path) {
            Ok(Cow::Borrowed(file))
        } else {
            Err(format!("File not found: {}", path).into())
        }
    }
}
