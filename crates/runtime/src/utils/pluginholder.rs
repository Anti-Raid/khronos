use mlua::prelude::*;

use crate::{
    plugins::{antiraid, lune},
    traits::context::KhronosContext,
};

/// A plugin set that can be used to load Khronos plugins into mlua::Lua.
///
/// This can be used to make plugin loading easier but is not strictly required
/// to use Khronos.
pub struct PluginSet {
    pub plugins: indexmap::IndexMap<String, fn(&Lua) -> LuaResult<LuaTable>>,
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
        //self.add_plugin("@antiraid/page", antiraid::page::init_plugin); [will be implemented later on, once we know how templates are being used]
        self.add_plugin("@antiraid/permissions", antiraid::permissions::init_plugin);
        self.add_plugin("@antiraid/promise", antiraid::promise::init_plugin);
        self.add_plugin("@antiraid/stings", antiraid::stings::init_plugin::<T>);
        self.add_plugin("@antiraid/datetime", antiraid::datetime::init_plugin);
        self.add_plugin("@antiraid/typesext", antiraid::typesext::init_plugin);
        self.add_plugin("@antiraid/userinfo", antiraid::userinfo::init_plugin::<T>);

        // External plugins
        self.add_plugin("@lune/datetime", lune::datetime::init_plugin);
        self.add_plugin("@lune/regex", lune::regex::init_plugin);
        self.add_plugin("@lune/serde", lune::serde::init_plugin);
        self.add_plugin("@lune/roblox", lune::roblox::init_plugin);
    }

    /// Adds a plugin to the plugin set.
    pub fn add_plugin(&mut self, name: impl ToString, function: fn(&Lua) -> LuaResult<LuaTable>) {
        self.plugins.insert(name.to_string(), function);
    }

    /// Requires a plugin by name.
    pub fn require(&self, lua: &Lua, plugin_name: String) -> LuaResult<LuaTable> {
        if let Ok(table) = lua.globals().get::<LuaTable>(plugin_name.clone()) {
            return Ok(table);
        }

        match self.plugins.get(plugin_name.as_str()) {
            Some(plugin) => plugin(lua),
            None => Err(LuaError::runtime(format!(
                "module '{}' not found",
                plugin_name
            ))),
        }
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
