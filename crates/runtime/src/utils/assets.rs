use std::{borrow::Cow, cell::RefCell, path::PathBuf, rc::Rc};

/// An asset manager is responsible for loading read-only assets.
///
/// This can/will be used in AntiRaid (at least) for multifile scripts
pub trait AssetManager {
    /// Gets the file contents given normalized path
    fn get_file(&self, path: &str) -> Result<impl AsRef<String>, crate::Error>;

    /// (optional) Gets a cached LuaValue, can be used to avoid repeated parsing of common things (like templating-types)
    fn get_cached_lua_value(&self, _: &mlua::Lua, _: &str) -> Option<mlua::MultiValue> {
        None
    }

    /// (optional) Clears all lua values if in a cache
    /// 
    /// Can be used to clear the cache of ``get_cached_lua_value``
    fn clear_cached_lua_values(&self) {}        
}

/// All Into<AssetManager> implementations should be able to be used as an AssetManager
impl<T: ?Sized> AssetManager for &T
where
    T: AssetManager,
{
    fn get_file(&self, path: &str) -> Result<impl AsRef<String>, crate::Error> {
        (**self).get_file(path)
    }
}

impl<T: AssetManager> AssetManager for Box<T> {
    fn get_file(&self, path: &str) -> Result<impl AsRef<String>, crate::Error> {
        (**self).get_file(path)
    }
}

impl<T: AssetManager> AssetManager for Rc<T> {
    fn get_file(&self, path: &str) -> Result<impl AsRef<String>, crate::Error> {
        (**self).get_file(path)
    }
}

#[derive(Clone)]
/// A simple fs-based asset manager for testing purposes
pub struct FileAssetManager {
    root_path: Rc<RefCell<PathBuf>>,
}

impl FileAssetManager {
    /// Creates a new `FileAssetManager` with the given root path.
    ///
    /// # Arguments
    ///
    /// * `root_path` - A string slice that holds the root path for the asset manager.
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path: Rc::new(RefCell::new(root_path)),
        }
    }

    /// Sets the root path for the file asset manager.
    pub fn set_root_path(&self, root_path: PathBuf) {
        if let Ok(mut path) = self.root_path.try_borrow_mut() {
            *path = root_path;
        }
    }
}

impl AssetManager for FileAssetManager {
    fn get_file(&self, path: &str) -> Result<impl AsRef<String>, crate::Error> {
        let path_ref = self
            .root_path
            .try_borrow()
            .map_err(|_| "Failed to borrow root path (concurrent access?)".to_string())?;

        let path = path_ref.join(path);
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
    fn get_file(&self, path: &str) -> Result<impl AsRef<String>, crate::Error> {
        log::debug!("[Require] Getting file: {}", path);
        if let Some(file) = self.assets.get(path) {
            Ok(Cow::Borrowed(file))
        } else {
            Err(format!("File not found: {}", path).into())
        }
    }
}
