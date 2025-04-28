use std::cell::RefCell;
use std::path::{Component, Path, PathBuf};
use std::collections::VecDeque;
use mlua::NavigateError;
use mlua::prelude::{Lua, LuaRequire, LuaTable, LuaValue, LuaResult, LuaFunction};
use std::io::Result as IoResult;
use sha2::{Sha256, Digest};
use std::rc::Rc;
use vfs::error::VfsErrorKind;
use vfs::{FileSystem, VfsError, VfsResult, SeekAndRead, SeekAndWrite, VfsMetadata, VfsPath};
use vfs::path::VfsFileType;
use std::fmt::Debug;
use std::time::SystemTime;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FilesystemWrapper(pub Arc<dyn FileSystem + Send + Sync>);

impl FilesystemWrapper {
    pub fn new<T: vfs::FileSystem>(fs: T) -> Self {
        Self(Arc::new(fs))
    }

    pub fn read_file(&self, path: &str) -> VfsResult<Vec<u8>> {
        let mut file = self.0.open_file(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }

    // Alias to read_file
    pub fn get_file(&self, path: &str) -> VfsResult<Vec<u8>> {
        self.read_file(path)
    }
}

impl FileSystem for FilesystemWrapper {
    fn read_dir(&self, path: &str) -> VfsResult<Box<dyn Iterator<Item = String> + Send>> {
        self.0.read_dir(&path)
    }

    fn create_dir(&self, path: &str) -> VfsResult<()> {
        self.0.create_dir(path)
    }

    fn open_file(&self, path: &str) -> VfsResult<Box<dyn SeekAndRead + Send>> {
        self.0.open_file(&path)
    }

    fn create_file(&self, path: &str) -> VfsResult<Box<dyn SeekAndWrite + Send>> {
        self.0.create_file(path)
    }

    fn append_file(&self, path: &str) -> VfsResult<Box<dyn SeekAndWrite + Send>> {
        self.0.append_file(path)
    }

    fn metadata(&self, path: &str) -> VfsResult<VfsMetadata> {
        self.0.metadata(&path)
    }

        fn set_creation_time(&self, path: &str, time: SystemTime) -> VfsResult<()> {
        self.0.set_creation_time(path, time)
    }

    fn set_modification_time(&self, _path: &str, _time: SystemTime) -> VfsResult<()> {
        self.0.set_modification_time(_path, _time)
    }

    fn set_access_time(&self, _path: &str, _time: SystemTime) -> VfsResult<()> {
        self.0.set_access_time(_path, _time)
    }

    fn exists(&self, path: &str) -> VfsResult<bool> {
        self.0.exists(&path)
    }

    fn remove_dir(&self, path: &str) -> VfsResult<()> {
        self.0.remove_dir(path)
    }

    fn remove_file(&self, path: &str) -> VfsResult<()> {
        self.0.remove_file(path)
    }

    fn copy_file(&self, _src: &str, _dest: &str) -> VfsResult<()> {
        self.0.copy_file(_src, _dest)
    }

    fn move_file(&self, _src: &str, _dest: &str) -> VfsResult<()> {
        self.0.move_file(_src, _dest)
    }

    fn move_dir(&self, _src: &str, _dest: &str) -> VfsResult<()> {
        self.0.move_dir(_src, _dest)
    }
}

#[derive(Clone)]
pub struct AssetRequirer {
    cache_prefix: Rc<RefCell<String>>,
    fs: FilesystemWrapper,
    global_table: LuaTable,
    abs_path: Rc<RefCell<PathBuf>>,
    rel_path: Rc<RefCell<PathBuf>>,
    suffix: Rc<RefCell<String>>,
}

// From https://github.com/mlua-rs/mlua/blob/main/src/luau/require.rs at branch a8a4aa8c930c9335b934c44bfea481f043b5ec3c
impl AssetRequirer {
    pub fn new(fs: FilesystemWrapper, cache_prefix: String, global_table: LuaTable) -> Self {
        Self {
            cache_prefix: RefCell::new(cache_prefix).into(),
            fs,
            global_table,
            abs_path: RefCell::default().into(),
            rel_path: RefCell::default().into(),
            suffix: RefCell::new("".to_string()).into(),
        }
    }

    fn path_fix(path: &Path) -> String {
        let mut path = path.to_string_lossy();
        if path.starts_with("./") {
            path = format!("/{}", path.trim_start_matches("./")).into();
        } else if !path.starts_with('/') {
            path = format!("/{}", path).into();
        }
        
        path.to_string()
    }

    fn is_file(&self, path: &Path) -> VfsResult<bool> {
        let path = Self::path_fix(path);

        log::trace!("Checking if {:#?} is a file", path);
        if !self.fs.exists(&path)? {
            log::trace!("File {:#?} does not exist", path);
            return Ok(false);
        }

        let metadata = self.fs.metadata(&path)?;
        Ok(metadata.file_type == VfsFileType::File)
    }

    fn get_file(&self, path: &Path) -> IoResult<Vec<u8>> {
        let path = Self::path_fix(path);

        let mut file = self.fs.open_file(&path).map_err(std::io::Error::other)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }

    fn is_dir(&self, path: &Path) -> VfsResult<bool> {
        let path = Self::path_fix(path);

        log::trace!("Checking if {:#?} is a directory", path);
        if path.is_empty() || path == "/" {
            return Ok(true);
        }

        if !self.fs.exists(&path)? {
            log::trace!("Directory {:#?} does not exist", path);
            return Ok(false);
        }

        let metadata = self.fs.0.metadata(&path)?;
        log::trace!("Metadata: {:#?}", metadata);
        Ok(metadata.file_type == VfsFileType::Directory)
    }
    
    // Normalizes the path by removing unnecessary components
    fn normalize_path(path: &Path) -> PathBuf {
        let mut components = VecDeque::new();

        for comp in path.components() {
            match comp {
                Component::Prefix(..) | Component::RootDir => {
                    components.push_back(comp);
                }
                Component::CurDir => {}
                Component::ParentDir => {
                    if matches!(components.back(), None | Some(Component::ParentDir)) {
                        components.push_back(Component::ParentDir);
                    } else if matches!(components.back(), Some(Component::Normal(..))) {
                        components.pop_back();
                    }
                }
                Component::Normal(..) => components.push_back(comp),
            }
        }

        if matches!(components.front(), None | Some(Component::Normal(..))) {
            components.push_front(Component::CurDir);
        }

        // Join the components back together
        components.into_iter().collect()
    }

    fn find_suffix(&self, path: &Path) -> Result<String, NavigateError> {
        log::trace!("Finding module path for {:#?}", path);
        let mut found = false;
        let mut suffix = "".to_string();

        let current_ext = (path.extension().and_then(|s| s.to_str()))
            .unwrap_or_default();
        for ext in ["luau", "lua"] {
            let candidate = if current_ext.is_empty() || current_ext == "lua" || current_ext == "luau" {
                path.with_extension(format!("{ext}"))
            } else {
                path.with_extension(format!("{current_ext}.{ext}"))
            };
            // log::trace!("Checking {:#?}", candidate);
            if self.is_file(&candidate).map_err(|_| NavigateError::NotFound)? {
                if found {
                    return Err(NavigateError::Ambiguous);
                }
                suffix = format!(".{ext}");
                found = true;
            }
        }

        if self.is_dir(path).map_err(|_| NavigateError::NotFound)? {
            if found {
                return Err(NavigateError::Ambiguous);
            }

            for component in ["/init.luau", "/init.lua"] {
                let candidate = path.join(component);
                if self.is_file(&candidate).map_err(|_| NavigateError::NotFound)? {
                    if found {
                        return Err(NavigateError::Ambiguous);
                    }
                    suffix = component.to_string();
                    found = true;
                }
            }

            found = true; 
        }

        if !found {
            return Err(NavigateError::NotFound);
        }

        Ok(suffix)
    }
}

impl LuaRequire for AssetRequirer {
    fn is_require_allowed(&self, _chunk_name: &str) -> bool {
        true
    }

    fn reset(&self, chunk_name: &str) -> Result<(), NavigateError> {
        log::trace!("Reset called with {chunk_name}");

        if chunk_name == "=repl" {
            self.abs_path.replace(PathBuf::from("/repl.luau"));
            self.rel_path.replace(PathBuf::from("./repl.luau"));
            self.suffix.replace("".to_string());

            return Ok(());
        }

        let path = Self::normalize_path(chunk_name.as_ref());

        if path.is_absolute() {
            log::trace!("Resetting to absolute path {:#?}", path);
            let suffix = self.find_suffix(&path)?;
            self.abs_path.replace(path.clone());
            self.rel_path.replace(path);
            self.suffix.replace(suffix);
        } else {
            // Relative path
            log::trace!("Resetting to relative path {:#?}", path);
            let cwd = PathBuf::from("/"); // TODO: Do we need anything special here?
            let abs_path = Self::normalize_path(&cwd.join(&path));
            let suffix = self.find_suffix(&abs_path)?;
            self.abs_path.replace(abs_path);
            self.rel_path.replace(path);
            self.suffix.replace(suffix);
        }

        // log::trace!("Resetting to {:#?}", self.abs_path.borrow());

        Ok(())
    }

    fn jump_to_alias(&self, path: &str) -> Result<(), NavigateError> {
        // log::trace!("Jumping to alias {path}");
        
        let path = Self::normalize_path(path.as_ref());
        let suffix = self.find_suffix(&path)?;

        self.abs_path.replace(path.clone());
        self.rel_path.replace(path);
        self.suffix.replace(suffix);

        Ok(())
    }

    fn to_parent(&self) -> Result<(), NavigateError> {
        log::trace!("Jumping to parent of {:#?}", self.abs_path.borrow());
        let mut abs_path = self.abs_path.borrow().clone();
        if abs_path.to_string_lossy().is_empty() || abs_path.to_string_lossy() == "." || abs_path.to_string_lossy() == "/" {
            log::trace!("No parent found for {:#?}", abs_path);
            return Err(NavigateError::NotFound);
        }
        if !abs_path.pop() {
            log::trace!("No parent found for {:#?}", abs_path);
            return Err(NavigateError::NotFound);
        }
        log::trace!("Parent is now {:#?}", abs_path);
        let mut rel_parent = self.rel_path.borrow().clone();
        rel_parent.pop();
        let suffix = self.find_suffix(&abs_path)?;

        self.abs_path.replace(abs_path);
        self.rel_path.replace(Self::normalize_path(&rel_parent));
        self.suffix.replace(suffix);

        Ok(())
    }

    fn to_child(&self, name: &str) -> Result<(), NavigateError> {
        // log::trace!("Jumping to child {:#?} with name {}", self.abs_path.borrow(), name);
        let abs_path = self.abs_path.borrow().join(name);
        let rel_path = self.rel_path.borrow().join(name);
        let suffix = self.find_suffix(&abs_path)?;
        // log::trace!("Found suffix {:#?}", suffix);

        self.abs_path.replace(abs_path);
        self.rel_path.replace(rel_path);
        self.suffix.replace(suffix);

        Ok(())
    }

    fn is_module_present(&self) -> bool {
        let suffix = self.suffix.borrow();
        let module_path = PathBuf::from(self.abs_path.borrow().to_string_lossy().to_string() + &*suffix);
        // log::trace!("Checking module exists {:#?}", module_path);
        self.is_file(&module_path)
            .unwrap_or(false)
    }

    fn contents(&self) -> IoResult<Vec<u8>> {
        let suffix = self.suffix.borrow();
        let module_path = PathBuf::from(self.abs_path.borrow().to_string_lossy().to_string() + &*suffix);
        // log::trace!("Loading contents for {:#?}", module_path);
        self.get_file(module_path.as_ref())
    }

    fn chunk_name(&self) -> String {
        self.rel_path.borrow().display().to_string()
    }

    fn cache_key(&self) -> Vec<u8> {
        // Try fetching the file itself to hash it directly
        if let Ok(contents) = self.contents() {
            // Take the sha256 hash of the contents
            let mut hasher = Sha256::new();
            hasher.update(contents);
            let hash = hasher.finalize();
            let mut cache_key = Vec::with_capacity(
                self.cache_prefix.borrow().len() + 1 + hash.len(),
            );
            cache_key.extend_from_slice(self.cache_prefix.borrow().as_bytes());
            cache_key.push(b'@');
            cache_key.extend_from_slice(&hash);
            log::trace!("Cache key: {:#?}", String::from_utf8_lossy(&cache_key));
            return cache_key;
        }

        // Otherwise, use the module path
        let cache_prefix = self.cache_prefix.borrow();
        let mut cache_key = Vec::with_capacity(
            cache_prefix.len() + 1 + self.abs_path.borrow().display().to_string().len() + self.suffix.borrow().len(),
        );
        cache_key.extend_from_slice(cache_prefix.as_bytes());
        cache_key.push(b'@');
        cache_key.extend_from_slice(self.abs_path.borrow().display().to_string().as_bytes());
        cache_key.extend_from_slice(self.suffix.borrow().as_bytes());
        log::trace!("Cache key: {:#?}", String::from_utf8_lossy(&cache_key));
        cache_key
    }

    fn is_config_present(&self) -> bool {
        let p = self.abs_path.borrow().join(".luaurc");
        self.is_file(p.as_ref()).unwrap_or(false)
    }

    fn config(&self) -> IoResult<Vec<u8>> {
        let p = self.abs_path.borrow().join(".luaurc");
        self.get_file(p.as_ref())
    }

    // Loads the module and returns the result (function or table).
    // For now, we don't support yielding in require'd modules due to luau require limitations.
    fn load(&self, lua: &Lua, _path: &str, chunk_name: &str, content: &[u8]) -> LuaResult<LuaValue> {
        log::trace!("Loading module {:#?}", chunk_name);
        let func = lua.load(content)
        .set_name(chunk_name)
        .set_environment(self.global_table.clone())
        .into_function()?;

        let th = lua.create_thread(func)?;

        let lv = th.resume(())?;

        let status = th.status();
        if status != mlua::ThreadStatus::Finished && status != mlua::ThreadStatus::Error {
            return Err(mlua::Error::runtime(format!("module '{}' attempted to yield within a require", chunk_name)));
        }

        Ok(lv)
    }
}

/// Test the require function
#[cfg(test)]
mod require_test {
    use mlua_scheduler::TaskManager;
    use mlua_scheduler_ext::feedbacks::ThreadTracker;
    use tokio::task::LocalSet;
    use mlua::IntoLua;

    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::time::Duration;

    fn create_luaurc_with_aliases(aliases: indexmap::IndexMap<String, String>) -> String {
        serde_json::to_string(&serde_json::json!({
            "aliases": aliases
        }))
        .expect("Failed to create luaurc")
    }

    #[test]
    fn test_basic_nested_require() {
        let mut tree = std::collections::HashMap::new();
        tree.insert(
            "init.luau".to_string(),
            "".to_string(),
        );
        tree.insert(
            "test.luau".to_string(),
            "return require('./foo/test')".to_string(),
        );
        tree.insert(
            "foo/test.luau".to_string(),
            "return require('./test2')".to_string(),
        );
        tree.insert(
            "foo/test2.luau".to_string(),
            "return require('./doo/test2')".to_string(),
        );
        tree.insert(
            "foo/doo/test2.luau".to_string(),
            "return require('@dir-alias/bar')".to_string(),
        );

        tree.insert(
            "foo/dir-alias/bar.luau".to_string(),
            "return require('./baz')".to_string(),
        );
        tree.insert(
            "foo/dir-alias/baz.luau".to_string(),
            "return require('@dir-alias/bat')".to_string(),
        );
        tree.insert(
            "foo/dir-alias/bat.luau".to_string(),
            "return require('./baz')".to_string(),
        );
        tree.insert(
            "foo/dir-alias/baz.luau".to_string(),
            "return require('../commacomma')".to_string(),
        );
        tree.insert(
            "foo/commacomma.luau".to_string(),
            "return require('./commacomma2')".to_string(),
        );
        tree.insert(
            "foo/commacomma2.luau".to_string(),
            "return require('../roothelper')".to_string(),
        );
        tree.insert(
            "roothelper.luau".to_string(),
            "return require('./roothelper2')".to_string(),
        );
        tree.insert(
            "roothelper2.luau".to_string(),
            "return require('@dir-alias-2/baz')".to_string(),
        );
        tree.insert(
            "dogs/2/baz.luau".to_string(),
            "return require('../../nextluaurcarea/baz')".to_string(),
        );
        tree.insert(
            "nextluaurcarea/baz.luau".to_string(),
            "return require('@dir-alias-2/chainy')".to_string(),
        );
        tree.insert("dogs/3/chainy.luau".to_string(), "return 3".to_string());

        tree.insert(
            ".luaurc".to_string(),
            create_luaurc_with_aliases(indexmap::indexmap! {
                "dir-alias".to_string() => "./foo/dir-alias".to_string(),
                "dir-alias-2".to_string() => "dogs/2".to_string()
            }),
        );
        tree.insert(
            "nextluaurcarea/.luaurc".to_string(),
            create_luaurc_with_aliases(indexmap::indexmap! {
                "dir-alias".to_string() => "../foo/dir-alias".to_string(),
                "dir-alias-2".to_string() => "../dogs/3".to_string()
            }),
        );

        let lua = mlua::Lua::new();

        let c = AssetRequirer::new(
            crate::utils::memoryvfs::create_vfs_from_map(tree).expect("Failed to make vfs"),
            "test".to_string(),
            lua.globals(),
        );

        lua.globals()
            .set(
                "require",
                lua.create_require_function(c).unwrap(),
            ) // Mock require
            .unwrap();

        let l: i32 = match lua.load("return require('./test')").set_name("/init").call(()) {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {e}");
                panic!("Failed to load test");
            }
        };
        assert_eq!(l, 3);
    }

    #[test]
    fn test_reqtest() {
        let lua = mlua::Lua::new();

        let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let c = super::FilesystemWrapper::new(
            vfs::PhysicalFS::new(
                base_path.join("tests"),
            )
        );

        let c = AssetRequirer::new(
            c,
            "reqtest".to_string(),
            lua.globals(),
        );

        lua.globals()
        .set(
            "require",
            lua.create_require_function(c).unwrap(),
        ) 
        .unwrap();
        
        let l: i32 = match lua.load("return require('./reqtest/a')").set_name("/init.luau").call(()) {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {e}");
                panic!("Failed to load test");
            }
        };

        assert_eq!(l, 1);
    }

    #[test]
    fn test_sythivo_a() {
        let main_luau = r#"
local foo = require("./foo/module")

assert(type(foo) == "function")
local res = foo();
assert(type(res) == "table")
print(res.resolved);
return res.resolved
        "#;

        let foo_module_luau = r#"
return function()
  return require("./test")
end
        "#;

        let foo_test_luau = r#"
return {
  resolved = true
}
        "#;

        let lua = mlua::Lua::new();

        let c = super::FilesystemWrapper::new(
            vfs::MemoryFS::new(),
        );

        c.create_dir("/foo").expect("Failed to create foo dir");
        c.create_file("/foo/module.luau").unwrap().write_all(foo_module_luau.as_bytes()).unwrap();
        c.create_file("/foo/test.luau").unwrap().write_all(foo_test_luau.as_bytes()).unwrap();
        c.create_file("/main.luau").unwrap().write_all(main_luau.as_bytes()).unwrap();

        let c = AssetRequirer::new(
            c,
            "styhivo_abc".to_string(),
            lua.globals(),
        );

        lua.globals()
        .set(
            "require",
            lua.create_require_function(c).unwrap(),
        ) 
        .unwrap();

        let func = lua.load(main_luau).set_name("/main.luau").into_function().unwrap();
        let th = lua.create_thread(func).unwrap();
        
        let l: bool = match th.resume(()) {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {e}");
                panic!("Failed to load test");
            }
        };

        assert_eq!(l, true);
    }
}
