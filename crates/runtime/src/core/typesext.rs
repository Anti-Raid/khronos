use std::{collections::HashMap, sync::Arc};

use mluau::prelude::*;
use mluau_require::AssetRequirer;
use rand::distr::{Alphanumeric, SampleString};

use crate::{primitives::opaque::Opaque, utils::{khronos_value::KhronosValue, proxyglobal::proxy_global}};

pub struct MemoryVfs {
    pub data: HashMap<String, String>,
}

impl MemoryVfs {
    pub fn new(data: HashMap<String, String>) -> Self {
        Self { data }
    }
}

impl LuaUserData for MemoryVfs {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("data", |lua, this, _: ()| {
            lua.to_value(&this.data)
        });
    }
}

#[derive(Debug, Clone)]
pub struct Vfs {
    pub vfs: Arc<mluau_require::Vfs>,

    #[allow(dead_code)]
    /// Not used currently, but may be useful in the future
    ///
    /// Denotes whether this VFS was created from an Opaque type
    /// 
    /// Will in future block certain operations that would expose the underlying data
    from_opaque: bool,
}

impl Vfs {
    pub fn new(vfs: Arc<mluau_require::Vfs>, opaque: bool) -> Self {
        Self { vfs, from_opaque: opaque }
    }
}

impl LuaUserData for Vfs {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("newoverlay", |_lua, vfs_list: Vec<LuaValue>| {
            let mut final_vfs = mluau_require::Vfs::new();
            let mut from_opaque = false;
            for vfs in vfs_list {
                match vfs {
                    LuaValue::UserData(vfs) => {
                        if vfs.is::<MemoryVfs>() {
                            let vfs = vfs
                            .borrow::<MemoryVfs>()
                            .map_err(|_| LuaError::external("Failed to borrow MemoryVfs"))?;

                            final_vfs.extend(mluau_require::create_memory_vfs_from_map_ref(&vfs.data));
                            continue;
                        } else if vfs.is::<Opaque>() {
                            let opaque = vfs
                            .borrow::<Opaque>()
                            .map_err(|_| LuaError::external("Failed to borrow Opaque"))?;
                            
                            let map = match &opaque.data {
                                KhronosValue::MemoryVfs(vfs) => vfs,
                                _ => return Err(LuaError::external("Opaque must contain a Vfs KhronosValue to be used as a VFS")),
                            };
                            final_vfs.extend(mluau_require::create_memory_vfs_from_map_ref(&map));
                            from_opaque = true; // taint as opaque
                            continue;
                        } else if vfs.is::<Vfs>() {
                            let vfs = vfs
                            .borrow::<Vfs>()
                            .map_err(|_| LuaError::external("Failed to borrow Vfs"))?;

                            final_vfs.extend_ref(&vfs.vfs);
                            continue;
                        } else {
                            return Err(LuaError::external(
                                "VFS list must contain only Vfs, MemoryVfs or Opaque(VFS) UserData",
                            ));
                        }
                    }
                    _ => {
                        return Err(LuaError::external(
                            "VFS list must contain only Vfs, MemoryVfs or Opaque(VFS) UserData",
                        ));
                    }
                }
            }

            Ok(Vfs::new(final_vfs.into(), from_opaque))
        });

        methods.add_method("createrequirefunction", |lua, this, (id, global_table): (String, LuaTable)| {
            let controller = AssetRequirer::new_arc(this.vfs.clone(), id, global_table);
            lua.create_require_function(controller)
        });
    }
}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "randstring",
        lua.create_function(|_lua, length: usize| {
            if length == 0 || length > 255 {
                return Err(LuaError::external(
                    "Length must be greater than 0 and less than 256",
                ));
            }

            Ok(Alphanumeric.sample_string(&mut rand::rng(), length))
        })?,
    )?;

    module.set("createvfs", lua.create_function(|lua, val: LuaValue| {
        let lazy_value: HashMap<String, String> = lua.from_value(val)
            .map_err(|e| LuaError::external(format!("Failed to convert LuaValue to serde_json::Value: {}", e)))?;

        Ok(MemoryVfs::new(lazy_value))
    })?)?;

    module.set("Vfs", lua.create_proxy::<Vfs>()?)?;

    module.set("createglobalproxy", lua.create_function(|lua, _: ()| {
        proxy_global(lua)
    })?)?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
