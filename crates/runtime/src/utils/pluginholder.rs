use mlua::prelude::*;

use crate::{
    plugins::{antiraid, lune},
    traits::context::KhronosContext,
};

#[derive(Clone, Copy, Debug)]
pub struct Plugin(fn(&Lua) -> LuaResult<LuaTable>);

impl From<fn(&Lua) -> LuaResult<LuaTable>> for Plugin {
    fn from(func: fn(&Lua) -> LuaResult<LuaTable>) -> Self {
        Self(func)
    }
}

impl LuaUserData for Plugin {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("load", |lua, this, ()| {
            let table = (this.0)(lua)?;
            Ok(table)
        });
    }
}

/// A plugin set that can be used to load Khronos plugins into mlua::Lua.
pub struct PluginSet {
    plugins: indexmap::IndexMap<String, Plugin>,
}

impl Default for PluginSet {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginSet {
    /// Creates a new plugin set.
    pub fn new() -> Self {
        Self {
            plugins: indexmap::IndexMap::new(),
        }
    }

    /// Adds the default Khronos plugins to the plugin set.
    pub fn add_default_plugins<T: KhronosContext>(&mut self) {
        // Antiraid plugins

        self.add_plugin("@antiraid/discord", antiraid::discord::init_plugin::<T>);
        self.add_plugin("@antiraid/interop", antiraid::interop::init_plugin);
        self.add_plugin("@antiraid/img_captcha", antiraid::img_captcha::init_plugin);
        self.add_plugin("@antiraid/kv", antiraid::kv::init_plugin::<T>);
        self.add_plugin("@antiraid/lazy", antiraid::lazy::init_plugin);
        self.add_plugin("@antiraid/lockdowns", antiraid::lockdowns::init_plugin::<T>);
        self.add_plugin("@antiraid/luau", antiraid::luau::init_plugin::<T>);
        self.add_plugin("@antiraid/pages", antiraid::pages::init_plugin::<T>);
        self.add_plugin("@antiraid/permissions", antiraid::permissions::init_plugin);
        self.add_plugin("@antiraid/promise", antiraid::promise::init_plugin);
        self.add_plugin("@antiraid/stings", antiraid::stings::init_plugin::<T>);
        self.add_plugin("@antiraid/datetime", antiraid::datetime::init_plugin);
        self.add_plugin("@antiraid/typesext", antiraid::typesext::init_plugin);
        self.add_plugin("@antiraid/userinfo", antiraid::userinfo::init_plugin::<T>);

        // External plugins
        self.add_plugin("@lune/datetime", lune::datetime::init_plugin);
        //self.add_plugin("@lune/regex", lune::regex::init_plugin);
        self.add_plugin("@lune/serde", lune::serde::init_plugin);
        self.add_plugin("@lune/roblox", lune::roblox::init_plugin);
    }

    /// Adds a plugin to the plugin set.
    pub fn add_plugin(&mut self, name: impl ToString, function: fn(&Lua) -> LuaResult<LuaTable>) {
        self.plugins.insert(name.to_string(), function.into());
    }

    /// Iterator over plugins
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Plugin)> {
        self.plugins.iter()
    }

    /// Requires a plugin by name.
    pub fn load_plugin(&self, lua: &Lua, plugin_name: &str) -> Option<LuaResult<LuaTable>> {
        if let Ok(table) = lua.globals().get::<LuaTable>(plugin_name) {
            return Some(Ok(table));
        }

        self.plugins.get(plugin_name).map(|plugin| (plugin.0)(lua))
    }
}

impl LuaUserData for PluginSet {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("load", |lua, this, plugin_name: String| {
            let Some(plugin) = this.load_plugin(lua, &plugin_name) else {
                return Err(mlua::Error::RuntimeError(format!(
                    "module '{}' not found",
                    plugin_name
                )));
            };

            plugin
        });
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_plugin_set() {
        pub static _KHRONOS_PLUGINSET: std::sync::LazyLock<super::PluginSet> =
            std::sync::LazyLock::new(|| {
                let mut plugins = super::PluginSet::new();
                plugins.add_default_plugins::<crate::traits::sample::SampleKhronosContext>();
                plugins
            });

        let mut my_plugin_set = super::PluginSet::new();
        my_plugin_set.add_plugin(
            "@antiraid/kv".to_string(),
            crate::plugins::antiraid::kv::init_plugin::<crate::traits::sample::SampleKhronosContext>,
        );

        my_plugin_set.add_plugin(
            "@antiraid/kv".to_string(),
            crate::plugins::antiraid::kv::init_plugin::<crate::traits::sample::SampleKhronosContext>,
        );

        my_plugin_set.add_default_plugins::<crate::traits::sample::SampleKhronosContext>();
    }
}
