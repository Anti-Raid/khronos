use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use mlua::prelude::*;
use super::vfs_navigator::{VfsNavigator, NavigationStatus};
use super::fswrapper::FilesystemWrapper;
use super::utils::is_absolute_path;
use std::io::Result as IoResult;

pub trait IntoNavError {
    fn into_nav_error(self) -> Result<(), LuaNavigateError>;
}

impl IntoNavError for Result<NavigationStatus, crate::Error> {
    fn into_nav_error(self) -> Result<(), LuaNavigateError> {
        match self {
            Ok(r) => {
                match r {
                    NavigationStatus::Success => Ok(()),
                    NavigationStatus::NotFound => Err(LuaNavigateError::NotFound),
                    NavigationStatus::Ambiguous => Err(LuaNavigateError::Ambiguous)
                }
            },
            Err(e) => {
                log::error!("Error while navigating: {:?}", e);
                Err(LuaNavigateError::NotFound)
            }
        }
    }
}

#[derive(Clone)]
pub struct AssetRequirer {
    cache_prefix: Rc<RefCell<String>>,
    vfs: Rc<RefCell<VfsNavigator>>,
    global_table: LuaTable,
}

impl AssetRequirer {
    pub fn new(fs: FilesystemWrapper, cache_prefix: String, global_table: LuaTable) -> Self {
        Self {
            cache_prefix: RefCell::new(cache_prefix).into(),
            vfs: Rc::new(RefCell::new(VfsNavigator::new(fs))),
            global_table,
        }
    }
}

impl LuaRequire for AssetRequirer {
    fn is_require_allowed(&self, _chunk_name: &str) -> bool {
        true
    }

    fn reset(&self, chunk_name: &str) -> Result<(), LuaNavigateError> {
        let mut vfs = self.vfs.try_borrow_mut().map_err(|e| {
            log::error!("Failed to borrow VFS: {:?}", e);
            LuaNavigateError::NotFound
        })?;

        if chunk_name == "=repl" {
            return vfs.reset_to_stdin().into_nav_error();
        }

        vfs.reset_to_path(&PathBuf::from(chunk_name)).into_nav_error()
    }

    fn jump_to_alias(&self, path: &str) -> Result<(), LuaNavigateError> {
        if !is_absolute_path(path) {
            return Err(LuaNavigateError::NotFound);
        }

        let mut vfs = self.vfs.try_borrow_mut().map_err(|e| {
            log::error!("Failed to borrow VFS: {:?}", e);
            LuaNavigateError::NotFound
        })?;

        log::trace!("Reset to alias: {}", path);

        vfs.reset_to_path(&PathBuf::from(path)).into_nav_error()
    }

    fn to_parent(&self) -> Result<(), LuaNavigateError> {
        let mut vfs = self.vfs.try_borrow_mut().map_err(|e| {
            log::error!("Failed to borrow VFS: {:?}", e);
            LuaNavigateError::NotFound
        })?;

        vfs.to_parent().into_nav_error()
    }
    
    fn to_child(&self, name: &str) -> Result<(), LuaNavigateError> {
        let mut vfs = self.vfs.try_borrow_mut().map_err(|e| {
            log::error!("Failed to borrow VFS: {:?}", e);
            LuaNavigateError::NotFound
        })?;

        vfs.to_child(name).into_nav_error()
    }

    fn is_module_present(&self) -> bool {
        let vfs = match self.vfs.try_borrow() {
            Ok(vfs) => vfs,
            Err(e) => {
                log::error!("Failed to borrow VFS: {:?}", e);
                return false;
            }
        };

        vfs.fs.is_file(vfs.get_file_path().to_string()).unwrap_or(false)
    }

    fn contents(&self) -> IoResult<Vec<u8>> {
        Ok(Vec::with_capacity(0)) // We load in load directly to avoid luau copy
    }

    fn chunk_name(&self) -> String {
        let vfs = match self.vfs.try_borrow() {
            Ok(vfs) => vfs,
            Err(e) => {
                log::error!("Failed to borrow VFS: {:?}", e);
                return "!".to_string();
            }
        };

        vfs.get_absolute_file_path().to_string()
    }

    fn cache_key(&self) -> Vec<u8> {
        let vfs = match self.vfs.try_borrow() {
            Ok(vfs) => vfs,
            Err(e) => {
                log::error!("Failed to borrow VFS: {:?}", e);
                return "!".to_string().into()
            }
        };

        let cache_prefix = self.cache_prefix.borrow();
        let mut cache_key = Vec::with_capacity(
            cache_prefix.len()
                + 1
                + vfs.get_absolute_file_path().len()
        );
        cache_key.extend_from_slice(cache_prefix.as_bytes());
        cache_key.push(b'@');
        cache_key.extend_from_slice(vfs.get_absolute_file_path().as_bytes());
        log::trace!("Cache key: {:#?}", String::from_utf8_lossy(&cache_key));
        cache_key
    }

    fn is_config_present(&self) -> bool {
        let vfs = match self.vfs.try_borrow() {
            Ok(vfs) => vfs,
            Err(e) => {
                log::error!("Failed to borrow VFS: {:?}", e);
                return false;
            }
        };

        vfs.fs.is_file(vfs.get_luaurc_path()).unwrap_or(false)
    }

    fn config(&self) -> IoResult<Vec<u8>> {
        let vfs = match self.vfs.try_borrow() {
            Ok(vfs) => vfs,
            Err(e) => {
                log::error!("Failed to borrow VFS: {:?}", e);
                return Ok("!".to_string().into());
            }
        };

        let luaurc_path = vfs.get_luaurc_path();

        log::trace!("Loading config from {:#?}", luaurc_path);
        vfs.fs.get_file(luaurc_path).map_err(std::io::Error::other)
    }

    fn loader(
        &self,
        lua: &Lua,
        _path: &str,
        chunk_name: &str,
        _content: &[u8],
    ) -> LuaResult<LuaFunction> {
        let content = {
            let vfs = self.vfs.try_borrow().map_err(LuaError::external)?;

            vfs.fs
            .get_file(chunk_name.to_string())
            .map_err(|e| mlua::Error::external(format!("Failed to fetch contents: {:?}", e)))?
        };

        let lv = lua
            .load(content)
            .set_mode(mlua::ChunkMode::Text)
            .set_name(chunk_name)
            .set_environment(self.global_table.clone())
            .into_function()?;

        Ok(lv)
    }
}