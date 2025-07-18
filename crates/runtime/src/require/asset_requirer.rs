use super::fswrapper::FilesystemWrapper;
use super::utils::is_absolute_path;
use super::vfs_navigator::{NavigationStatus, VfsNavigator};
use mluau::prelude::*;
use std::io::Result as IoResult;
use std::path::PathBuf;

pub trait IntoNavError {
    fn into_nav_error(self) -> Result<(), LuaNavigateError>;
}

impl IntoNavError for Result<NavigationStatus, crate::Error> {
    fn into_nav_error(self) -> Result<(), LuaNavigateError> {
        match self {
            Ok(r) => match r {
                NavigationStatus::Success => Ok(()),
                NavigationStatus::NotFound => Err(LuaNavigateError::NotFound),
                NavigationStatus::Ambiguous => Err(LuaNavigateError::Ambiguous),
            },
            Err(e) => Err(LuaNavigateError::Other(LuaError::external(format!(
                "Navigation error: {e}",
            )))),
        }
    }
}

pub struct AssetRequirer {
    cache_prefix: String,
    vfs: VfsNavigator,
    global_table: LuaTable,
}

impl AssetRequirer {
    pub fn new(fs: FilesystemWrapper, cache_prefix: String, global_table: LuaTable) -> Self {
        Self {
            cache_prefix,
            vfs: VfsNavigator::new(fs),
            global_table,
        }
    }
}

impl LuaRequire for AssetRequirer {
    fn is_require_allowed(&self, _chunk_name: &str) -> bool {
        true
    }

    fn reset(&mut self, chunk_name: &str) -> Result<(), LuaNavigateError> {
        if chunk_name == "=repl" {
            return self.vfs.reset_to_stdin().into_nav_error();
        }

        self.vfs
            .reset_to_path(&PathBuf::from(chunk_name))
            .into_nav_error()
    }

    fn jump_to_alias(&mut self, path: &str) -> Result<(), LuaNavigateError> {
        if !is_absolute_path(path) {
            return Err(LuaNavigateError::NotFound);
        }

        log::trace!("Reset to alias: {path}");

        self.vfs
            .reset_to_path(&PathBuf::from(path))
            .into_nav_error()
    }

    fn to_parent(&mut self) -> Result<(), LuaNavigateError> {
        self.vfs.to_parent().into_nav_error()
    }

    fn to_child(&mut self, name: &str) -> Result<(), LuaNavigateError> {
        self.vfs.to_child(name).into_nav_error()
    }

    fn has_module(&self) -> bool {
        self.vfs
            .fs
            .is_file(self.vfs.get_file_path().to_string())
            .unwrap_or(false)
    }

    fn cache_key(&self) -> String {
        format!(
            "{}@{}",
            self.cache_prefix,
            self.vfs.get_absolute_file_path()
        )
    }

    fn has_config(&self) -> bool {
        self.vfs
            .fs
            .is_file(self.vfs.get_luaurc_path())
            .unwrap_or(false)
    }

    fn config(&self) -> IoResult<Vec<u8>> {
        let luaurc_path = self.vfs.get_luaurc_path();

        log::trace!("Loading config from {luaurc_path:#?}");
        self.vfs
            .fs
            .get_file(luaurc_path)
            .map_err(std::io::Error::other)
    }

    fn loader(&self, lua: &Lua) -> LuaResult<LuaFunction> {
        let chunk_name = self.vfs.get_absolute_file_path();
        let content = self
            .vfs
            .fs
            .get_file(chunk_name.to_string())
            .map_err(|e| mluau::Error::external(format!("Failed to fetch contents: {e:?}")))?;

        let lv = lua
            .load(content)
            .set_mode(mluau::ChunkMode::Text)
            .set_name(chunk_name)
            .set_environment(self.global_table.clone())
            .into_function()?;

        Ok(lv)
    }
}
