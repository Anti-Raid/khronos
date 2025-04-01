use std::{
    collections::HashMap,
    path::{Component, Path, PathBuf},
};

use mlua::prelude::*;
use mlua_scheduler::LuaSchedulerAsync;
use mlua_scheduler_ext::{traits::IntoLuaThread, Scheduler};

/// The code that is run for requires
pub const REQUIRE_LUAU_ASYNC_CODE: &str = r#"
local luacall, pluginreg = ...

local pluginregreqcache = {}
local function callback(path, ...)
    if not path then
        error("missing argument #1 to 'require' (string expected)")
    end

    -- Fast path
    if pluginreg[path] then
        if pluginregreqcache[path] then
            return pluginregreqcache[path]
        end
        local plugin = pluginreg[path]:load(...)
        pluginregreqcache[path] = plugin
        return plugin
    end

    local debugname = debug.info(2, "s")
    luacall(coroutine.running(), string.sub(debugname, 10, -3), path, ...)
    return coroutine.yield()
end

return callback
"#;

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
pub struct LuauRcFile {
    #[serde(default)]
    aliases: HashMap<String, PathBuf>,
}

pub struct LuauRc {
    aliases: HashMap<String, (PathBuf, PathBuf)>,
}

impl LuauRc {
    fn new() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }

    fn merge(&mut self, dir: &Path, other: LuauRcFile) {
        for (key, value) in other.aliases {
            self.aliases
                .entry(key)
                .or_insert((dir.to_path_buf(), value));
        }
    }
}

/// luaurc is the standard way to handle aliases in Luau
///
/// Starts at current dir and recurses down until we reach the root merging as we go
pub fn look_for_luaurc<T: RequireController>(
    dir: impl AsRef<Path>,
    require_controller: &T,
) -> LuauRc {
    let mut dir = dir.as_ref();

    let mut luaurc = LuauRc::new();
    loop {
        log::info!("In directory for require resolution: {:?}", dir);
        // Keep recursing down from current dir
        let luaurc_path = dir.join(".luaurc");
        if let Ok(luaurc_file) = require_controller.get_file(&luaurc_path.to_string_lossy()) {
            if let Ok(luaurc_new) = serde_json5::from_str(luaurc_file.as_ref()) {
                luaurc.merge(dir, luaurc_new);
                break; // For now, until Luau team makes a further RFC which is being waited on, stop at first luaurc found
            }
        }

        if let Some(parent) = dir.parent() {
            dir = parent;
            log::info!(
                "Trying to find .luaurc in: {:?} for require resolution",
                dir
            );
        } else {
            break;
        }
    }

    luaurc
}

/// Controller to be used by `require`
///
/// Note that controllers should not be reused across script invocations
pub trait RequireController {
    /// Returns a builtins table
    fn get_builtins(&self) -> Option<LuaResult<LuaTable>>;

    /// Returns a builtin
    fn get_builtin(&self, builtin: &str) -> Option<LuaResult<LuaMultiValue>>;

    /// Gets the file contents given normalized path
    fn get_file(&self, path: &str) -> Result<impl AsRef<String>, crate::Error>;

    /// Returns a LuaMultiValue from the cache (if any)
    fn get_cached(&self, path: &str) -> Option<LuaMultiValue>;

    /// Caches the file contents
    fn cache(&self, path: String, contents: LuaMultiValue);

    /// Returns the global table to provide to the required file
    fn global_table(&self) -> LuaTable;
}

/// Require a file with require-by-string semantics from a given controller
///
/// You probably want `create_require_function` instead though in practice
pub async fn require_from_controller<T: RequireController>(
    lua: &Lua,
    pat: String,
    controller: impl AsRef<T>,
    chunk_name: String,
) -> LuaResult<LuaMultiValue> {
    let controller = controller.as_ref();

    // Builtins override all else
    if let Some(builtin) = controller.get_builtin(&pat) {
        return builtin;
    }

    // require path must start with a valid prefix: ./, ../ or @ for rbs
    if !pat.starts_with("./") && !pat.starts_with("../") && !pat.starts_with("@") {
        return Err(LuaError::external(format!(
            "Invalid require path: {}. Must start with ./, ../ or @ to comply with luau require-by-string semantics",
            pat
        )));
    }

    let chunkname = {
        if chunk_name.starts_with("./") {
            chunk_name
        } else {
            "".to_string() // default to empty string
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

        let luaurc = look_for_luaurc(&curr_path, controller);

        // Aliases have special resolution logic
        match luaurc.aliases.get(parts[0].trim_start_matches('@')) {
            Some((dir, p)) => {
                let path = normalize_path(&dir.join(p));

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
    let th = lua
        .load(file_contents.as_ref())
        .set_name(format!("./{}", pat))
        .set_environment(controller.global_table())
        .into_lua_thread(lua)?;

    let scheduler = Scheduler::get(lua);
    let ret = scheduler
        .spawn_thread_and_wait("Spawn", th, LuaMultiValue::new())
        .await?;

    match ret {
        Some(Ok(ret)) => {
            controller.cache(pat.clone(), ret.clone());
            Ok(ret)
        }
        Some(Err(ret)) => Err(ret),
        None => Ok(LuaMultiValue::with_capacity(0)),
    }
}

pub fn create_require_function<T: RequireController>(
    lua: &Lua,
    controller_ref: impl AsRef<T> + Clone + 'static,
) -> LuaResult<LuaFunction> {
    let builtins = match controller_ref.as_ref().get_builtins() {
        Some(builtins) => builtins?,
        None => lua.create_table()?,
    };

    let args = {
        let mut args = LuaMultiValue::with_capacity(1);
        args.push_back(LuaValue::Table(builtins));
        args
    };

    lua.create_scheduler_async_function_with(
        move |lua, (chunk_name, pat): (String, String)| {
            let controller_ref = controller_ref.clone();
            async move { require_from_controller(&lua, pat, &controller_ref, chunk_name).await }
        },
        REQUIRE_LUAU_ASYNC_CODE,
        args,
    )
}

/// Test the require function
#[cfg(test)]
mod require_test {
    use mlua_scheduler::TaskManager;
    use mlua_scheduler_ext::feedbacks::ThreadTracker;
    use tokio::task::LocalSet;

    use crate::utils::assets::{AssetManager, FileAssetManager, HashMapAssetManager};

    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::time::Duration;

    struct SimpleRequireController<T: AssetManager> {
        asset_manager: T,
        requires_cache: RefCell<std::collections::HashMap<String, LuaMultiValue>>,
        lua: Lua,
    }

    impl<T: AssetManager> SimpleRequireController<T> {
        pub fn new(asset_manager: T, lua: Lua) -> Self {
            Self {
                asset_manager,
                requires_cache: RefCell::new(std::collections::HashMap::new()),
                lua,
            }
        }
    }

    impl<T: AssetManager> RequireController for SimpleRequireController<T> {
        fn get_builtins(&self) -> Option<LuaResult<LuaTable>> {
            None
        }

        fn get_builtin(&self, _builtin: &str) -> Option<LuaResult<LuaMultiValue>> {
            None
        }

        fn get_file(&self, path: &str) -> Result<impl AsRef<String>, crate::Error> {
            self.asset_manager.get_file(path)
        }

        fn get_cached(&self, path: &str) -> Option<LuaMultiValue> {
            if let Ok(requires_cache) = self.requires_cache.try_borrow() {
                return requires_cache.get(path).cloned();
            }
            None
        }

        fn cache(&self, path: String, contents: LuaMultiValue) {
            if let Ok(mut requires_cache) = self.requires_cache.try_borrow_mut() {
                requires_cache.insert(path, contents);
            }
        }

        fn global_table(&self) -> LuaTable {
            self.lua.globals()
        }
    }

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

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let localset = LocalSet::new();
        localset.block_on(&rt, async move {
            let lua = mlua::Lua::new();

            let controller = {
                let c = SimpleRequireController::new(HashMapAssetManager::new(tree), lua.clone());

                Rc::new(c)
            };
            let controller_b = controller.clone();

            let tt = ThreadTracker::new();
            let scheduler = Scheduler::new(TaskManager::new(
                lua.clone(),
                Rc::new(tt.clone()),
                Duration::from_micros(1),
            ));
            lua.set_app_data(tt);
            scheduler.attach();
            lua.globals()
                .set(
                    "require",
                    create_require_function(&lua, controller_b.clone()).unwrap(),
                ) // Mock require
                .unwrap();
            let pat = "./test".to_string();
            let ret = super::require_from_controller(&lua, pat, &controller, "".to_string()).await;
            assert!(mvr_is_v(&lua, &ret, 3));
        });
    }

    #[test]
    fn test_reqtest() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let localset = LocalSet::new();
        localset.block_on(&rt, async move {
            let lua = mlua::Lua::new();

            let tt = ThreadTracker::new();
            let scheduler = Scheduler::new(TaskManager::new(
                lua.clone(),
                Rc::new(tt.clone()),
                Duration::from_micros(1),
            ));
            lua.set_app_data(tt);
            scheduler.attach();

            let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

            let controller = {
                let c = SimpleRequireController::new(FileAssetManager::new(
                    base_path.join("tests"),
                    lua.clone(),
                ));

                Rc::new(c)
            };

            let controller_b = controller.clone();

            lua.globals()
                .set(
                    "require",
                    create_require_function(&lua, controller_b.clone()).unwrap(),
                ) // Mock require
                .unwrap();
            let pat = "./reqtest/a".to_string();
            let ret = super::require_from_controller(&lua, pat, &controller, "".to_string()).await;
            assert!(mvr_is_v(&lua, &ret, 1));
        });
    }
}
