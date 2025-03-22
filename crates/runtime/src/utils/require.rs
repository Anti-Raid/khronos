use std::{
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    path::{Component, Path, PathBuf},
};

use mlua::prelude::*;

// From Cargo
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

#[derive(serde::Deserialize)]
pub struct LuauRc {
    #[serde(skip_serializing_if = "Option::is_none")]
    aliases: Option<HashMap<String, PathBuf>>,
}

/// luaurc is the standard way to handle aliases in Luau
pub fn look_for_luaurc<T: RequireController>(
    dir: impl AsRef<Path>,
    require_controller: &T,
) -> Option<LuauRc> {
    let mut dir = dir.as_ref();

    // Try cache first
    if let Some(cached_luaurc_path) = require_controller.get_cached_luaurc(dir) {
        if let Ok(luaurc_file) = require_controller.get_file(&cached_luaurc_path.to_string_lossy())
        {
            if let Ok(luaurc) = serde_json5::from_str(&luaurc_file) {
                return Some(luaurc);
            }
        }
    }

    loop {
        // Keep recursing down from current dir
        let luaurc_path = dir.join(".luaurc");
        if let Ok(luaurc_file) = require_controller.get_file(&luaurc_path.to_string_lossy()) {
            if let Ok(luaurc) = serde_json5::from_str(&luaurc_file) {
                require_controller.cache_luaurc(dir.to_path_buf(), luaurc_path);
                return Some(luaurc);
            }
        }

        dir = dir.parent()?;
    }
}

/// Controller to be used by `require`
///
/// Note that controllers should not be reused across script invocations
pub trait RequireController {
    /// Returns a builtin
    fn get_builtin(&self, builtin: &str) -> Option<LuaMultiValue>;

    /// Gets the file contents given normalized path
    fn get_file(&self, path: &str) -> Result<Cow<'_, str>, crate::Error>;

    /// Returns a LuaMultiValue from the cache (if any)
    fn get_cached(&self, path: &str) -> Option<LuaMultiValue>;

    /// Caches the file contents
    fn cache(&self, path: String, contents: LuaMultiValue);

    /// Returns a cached Luaurc path for a given path
    fn get_cached_luaurc(&self, path: &Path) -> Option<PathBuf>;

    /// Caches a luaurc
    fn cache_luaurc(&self, path: PathBuf, luaurc_path: PathBuf);
}

/// Require a file with require-by-string semantics from a given controller
pub fn require_from_controller<T: RequireController>(
    lua: &Lua,
    pat: String,
    controller: impl AsRef<T>,
    callstack_level: Option<usize>,
) -> LuaResult<LuaMultiValue> {
    let controller = controller.as_ref();

    // Builtins override all else
    if let Some(builtin) = controller.get_builtin(&pat) {
        return Ok(builtin);
    }

    // require path must start with a valid prefix: ./, ../ or @ for rbs
    if !pat.starts_with("./") && !pat.starts_with("../") && !pat.starts_with("@") {
        return Err(LuaError::external(format!(
            "Invalid require path: {}. Must start with ./, ../ or @ to comply with luau require-by-string semantics",
            pat
        )));
    }

    let chunkname = {
        let stack = lua.inspect_stack(callstack_level.unwrap_or(1)); // 1 is the function that called require
        if let Some(stack) = stack {
            if let Some(chunkname) = stack.source().source {
                if chunkname.starts_with("./") {
                    chunkname.to_string()
                } else {
                    "".to_string() // default to empty string
                }
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    };

    let chunkname = chunkname.trim_start_matches("/").trim_start_matches("./");

    let curr_path = {
        let mut chunk_path = PathBuf::from(chunkname);
        chunk_path.pop(); // Remove the file name from path
        chunk_path
    };

    log::debug!("Current path: {:?} when requiring {}", curr_path, pat);
    let pat = if pat.starts_with('@') {
        // Split the path into alias and file
        let parts = pat
            .splitn(2, '/')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(LuaError::external(format!(
                "Invalid require path: {}. Must be in the format @alias/file",
                pat
            )));
        }

        let Some(luaurc) = look_for_luaurc(&curr_path, controller) else {
            return Err(LuaError::external(format!(
                "Invalid require path: {}. No valid .luaurc file found",
                pat
            )));
        };

        let Some(aliases) = luaurc.aliases else {
            return Err(LuaError::external(format!(
                "Invalid require path: {}. No aliases found in closest .luaurc file",
                pat
            )));
        };

        // Aliases have special resolution logic
        match aliases.get(&parts[0]) {
            Some(p) => {
                let path = normalize_path(p);

                if parts.len() == 2 {
                    path.join(&parts[1])
                } else {
                    path
                }
            }
            None => {
                return Err(LuaError::external(format!(
                    "Failed to resolve alias {}",
                    pat
                )));
            }
        }
    } else {
        normalize_path(&curr_path.join(&pat))
    };

    log::debug!(
        "Resolved: Current path: {:?} when requiring {}",
        curr_path,
        pat.display()
    );

    if pat.ends_with(".luau") {
        return Err(LuaError::external(format!(
            "Failed to load module '{}': .luau extension must be removed to comply with luau require-by-string semantics",
            pat.display()
        )));
    }

    let pat = format!("{}.luau", pat.to_string_lossy()).to_string();

    let mut file_contents = None;
    if let Ok(file) = controller.get_file(&pat) {
        file_contents = Some(file);
    };

    let Some(file_contents) = file_contents else {
        return Err(LuaError::external(format!("module '{}' not found", pat)));
    };

    if let Some(cached) = controller.get_cached(&pat) {
        log::debug!("[Require] Cached: {:?}", cached);
        return Ok(cached.clone());
    }

    // Execute the file
    let ret = lua
        .load(&*file_contents)
        .set_name(format!("./{}", pat))
        .eval::<LuaMultiValue>();

    if let Ok(ret) = ret {
        // Cache the result
        controller.cache(pat, ret.clone());

        return Ok(ret);
    }

    ret
}

/// A simple require controller for simple usecases and testing
pub struct TestRequireController {
    requires_cache: RefCell<std::collections::HashMap<String, LuaMultiValue>>,
    fstree: std::collections::HashMap<String, String>,
    cached_luaurc: RefCell<std::collections::HashMap<PathBuf, PathBuf>>,
}

impl TestRequireController {
    pub fn new(tree: std::collections::HashMap<String, String>) -> Self {
        Self {
            requires_cache: RefCell::new(std::collections::HashMap::new()),
            fstree: tree,
            cached_luaurc: RefCell::new(std::collections::HashMap::new()),
        }
    }
}

impl RequireController for TestRequireController {
    fn get_builtin(&self, _builtin: &str) -> Option<LuaMultiValue> {
        None
    }

    fn get_file(&self, path: &str) -> Result<Cow<'_, str>, crate::Error> {
        println!("[Require] Getting file: {}", path);
        if let Some(file) = self.fstree.get(path) {
            Ok(Cow::Borrowed(file))
        } else {
            Err(format!("File not found: {}", path).into())
        }
    }

    fn get_cached(&self, path: &str) -> Option<LuaMultiValue> {
        self.requires_cache.borrow().get(path).cloned()
    }

    fn cache(&self, path: String, contents: LuaMultiValue) {
        self.requires_cache.borrow_mut().insert(path, contents);
    }

    fn get_cached_luaurc(&self, path: &Path) -> Option<PathBuf> {
        self.cached_luaurc.borrow().get(path).cloned()
    }

    fn cache_luaurc(&self, path: PathBuf, luaurc_path: PathBuf) {
        self.cached_luaurc.borrow_mut().insert(path, luaurc_path);
    }
}

/// A simple require controller for simple usecases and testing using a filesystem as fstree
pub struct TestFilesystemRequireController {
    requires_cache: RefCell<std::collections::HashMap<String, LuaMultiValue>>,
    root_path: PathBuf,
    cached_luaurc: RefCell<std::collections::HashMap<PathBuf, PathBuf>>,
}

impl TestFilesystemRequireController {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            requires_cache: RefCell::new(std::collections::HashMap::new()),
            root_path,
            cached_luaurc: RefCell::new(std::collections::HashMap::new()),
        }
    }
}

impl RequireController for TestFilesystemRequireController {
    fn get_builtin(&self, _builtin: &str) -> Option<LuaMultiValue> {
        None
    }

    fn get_file(&self, path: &str) -> Result<Cow<'_, str>, crate::Error> {
        println!("Current dir: {:?}", std::env::current_dir());

        let path = self.root_path.join(path);
        println!("[RequireFS] Getting file: {}", path.display());
        if let Ok(file) = std::fs::read_to_string(&path) {
            return Ok(Cow::Owned(file));
        }

        Err(format!("File not found: {}", path.display()).into())
    }

    fn get_cached(&self, path: &str) -> Option<LuaMultiValue> {
        self.requires_cache.borrow().get(path).cloned()
    }

    fn cache(&self, path: String, contents: LuaMultiValue) {
        self.requires_cache.borrow_mut().insert(path, contents);
    }

    fn get_cached_luaurc(&self, path: &Path) -> Option<PathBuf> {
        self.cached_luaurc.borrow().get(path).cloned()
    }

    fn cache_luaurc(&self, path: PathBuf, luaurc_path: PathBuf) {
        self.cached_luaurc.borrow_mut().insert(path, luaurc_path);
    }
}

/// Test the require function
#[cfg(test)]
mod require_test {
    use super::*;
    use std::rc::Rc;

    fn mv_is_v(lua: &Lua, mv: &LuaMultiValue, v: impl IntoLua) -> bool {
        let v = v.into_lua(lua).unwrap();
        for i in mv.iter() {
            if i == &v {
                return true;
            }
        }
        false
    }

    fn mvr_is_v(lua: &Lua, mv: &Result<LuaMultiValue, LuaError>, v: impl IntoLua) -> bool {
        mv_is_v(lua, mv.as_ref().unwrap(), v)
    }

    fn create_luaurc_with_aliases(aliases: indexmap::IndexMap<String, String>) -> String {
        serde_json::to_string(&serde_json::json!({
            "aliases": aliases
        }))
        .expect("Failed to create luaurc")
    }

    #[test]
    fn test_basic_require() {
        let mut tree = std::collections::HashMap::new();
        tree.insert("test.luau".to_string(), "return 2".to_string());
        tree.insert("test2.luau".to_string(), "return 3".to_string());

        let controller = Rc::new(TestRequireController::new(tree));

        let lua = mlua::Lua::new();
        let pat = "./test".to_string();
        let ret = super::require_from_controller(&lua, pat, &controller, None);
        assert!(mvr_is_v(&lua, &ret, 2));
    }

    #[test]
    fn test_chunkname_extraction() {
        let lua = mlua::Lua::new();
        lua.globals()
            .set(
                "getchunkname",
                lua.create_function(|lua, _: ()| {
                    let stack = lua.inspect_stack(1);
                    let Some(stack) = stack else {
                        return Err(LuaError::external("getchunkname failed"));
                    };

                    let Some(chunkname) = stack.source().source else {
                        return Err(LuaError::external(
                            "Attempt to call getchunkname outside a chunk",
                        ));
                    };

                    println!("{}", chunkname);
                    Ok(chunkname.to_string())
                })
                .unwrap(),
            ) // Mock require
            .unwrap();

        lua.load("assert(getchunkname() == 'mycoolchunk')")
            .set_name("mycoolchunk")
            .eval::<()>()
            .expect("Failed to getchunkname");

        lua.load("assert(getchunkname() == 'mycoolchunk2')")
            .set_name("mycoolchunk2")
            .eval::<()>()
            .expect("Failed to getchunkname");
    }

    #[test]
    fn test_basic_nested_require() {
        let mut tree = std::collections::HashMap::new();
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
                "@dir-alias".to_string() => "./foo/dir-alias".to_string(),
                "@dir-alias-2".to_string() => "dogs/2".to_string()
            }),
        );
        tree.insert(
            "nextluaurcarea/.luaurc".to_string(),
            create_luaurc_with_aliases(indexmap::indexmap! {
                "@dir-alias".to_string() => "./foo/dir-alias".to_string(),
                "@dir-alias-2".to_string() => "dogs/3".to_string()
            }),
        );

        let controller = {
            let c = TestRequireController::new(tree);

            Rc::new(c)
        };
        let controller_b = controller.clone();

        let lua = mlua::Lua::new();
        lua.globals()
            .set(
                "require",
                lua.create_function(move |lua, pat: String| {
                    super::require_from_controller(lua, pat, &controller_b, None)
                })
                .unwrap(),
            ) // Mock require
            .unwrap();
        let pat = "./test".to_string();
        let ret = super::require_from_controller(&lua, pat, &controller, None);
        assert!(mvr_is_v(&lua, &ret, 3));
    }

    #[test]
    fn test_reqtest() {
        let lua = mlua::Lua::new();

        let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let controller = {
            Rc::new(TestFilesystemRequireController::new(
                base_path.join("tests"),
            ))
        };

        let controller_b = controller.clone();

        lua.globals()
            .set(
                "require",
                lua.create_function(move |lua, pat: String| {
                    super::require_from_controller(lua, pat, &controller_b, None)
                })
                .unwrap(),
            ) // Mock require
            .unwrap();
        let pat = "./reqtest/a".to_string();
        let ret = super::require_from_controller(&lua, pat, &controller, None);
        assert!(mvr_is_v(&lua, &ret, 1));
    }
}
