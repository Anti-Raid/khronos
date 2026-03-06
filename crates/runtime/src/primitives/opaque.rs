use mluau::prelude::*;

use crate::utils::khronos_value::KhronosValue;

/// Represents an opaque blob that is not revealed to Luau whatsoever.
/// 
/// This is mainly useful with global kv's which can be converted into VFS's
/// that can be require'd from using Vfs:createrequirefunction without leaking
/// the underlying source code/data to Luau.
pub struct Opaque {
    pub data: KhronosValue,
}

impl Opaque {
    pub fn new(data: KhronosValue) -> Self {
        Self {
            data,
        }
    }
}

impl std::fmt::Debug for Opaque {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Opaque").finish()
    }
}

impl LuaUserData for Opaque {}