use mluau::prelude::*;

/// Represents an opaque blob that is not revealed to Luau whatsoever.
/// 
/// This is mainly useful with global kv's which can be converted into VFS's
/// that can be require'd from using Vfs:createrequirefunction without leaking
/// the underlying source code/data to Luau.
pub struct Opaque<T: 'static> {
    pub data: T,
}

impl<T: 'static> Opaque<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
        }
    }
}

// A T can be converted to a Opaque<T> by just wrapping it
impl<T> From<T> for Opaque<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<T: Clone> Clone for Opaque<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Opaque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Opaque").finish()
    }
}

// A Opaque<T> is a LuaUserData
impl<T: 'static> LuaUserData for Opaque<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "Opaque");
    }

    #[cfg(feature = "repl")]
    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
    }
}
