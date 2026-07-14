use std::{collections::HashMap, sync::Arc};

use mluau::prelude::*;
use mluau_require::{AssetRequirer, FilesystemWrapper};
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
    pub vfs: Arc<dyn mluau_require::vfs::FileSystem>,

    #[allow(dead_code)]
    /// Not used currently, but may be useful in the future
    ///
    /// Denotes whether this VFS was created from an Opaque type
    /// 
    /// Will in future block certain operations that would expose the underlying data
    from_opaque: bool,
}

impl Vfs {
    pub fn new(vfs: Arc<dyn mluau_require::vfs::FileSystem>, opaque: bool) -> Self {
        Self { vfs, from_opaque: opaque }
    }
}

impl LuaUserData for Vfs {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("newoverlay", |_lua, vfs_list: Vec<LuaValue>| {
            let mut vfs_refs = Vec::with_capacity(vfs_list.len());
            let mut from_opaque = false;
            for vfs in vfs_list {
                match vfs {
                    LuaValue::UserData(vfs) => {
                        if vfs.is::<MemoryVfs>() {
                            let vfs = vfs
                            .borrow::<MemoryVfs>()
                            .map_err(|_| LuaError::external("Failed to borrow MemoryVfs"))?;

                            vfs_refs.push(mluau_require::vfs::VfsPath::new(
                                mluau_require::create_memory_vfs_from_map(&vfs.data)
                                .map_err(|e| LuaError::external(format!("Failed to create memory VFS: {}", e)))?,
                            ));
                            continue;
                        } else if vfs.is::<Opaque>() {
                            let opaque = vfs
                            .borrow::<Opaque>()
                            .map_err(|_| LuaError::external("Failed to borrow Opaque"))?;
                            
                            let map = match &opaque.data {
                                KhronosValue::MemoryVfs(vfs) => vfs,
                                _ => return Err(LuaError::external("Opaque must contain a Vfs KhronosValue to be used as a VFS")),
                            };

                            vfs_refs.push(mluau_require::vfs::VfsPath::new(
                                mluau_require::create_memory_vfs_from_map(&map)
                                .map_err(|e| LuaError::external(format!("Failed to create memory VFS: {}", e)))?,
                            ));
                            from_opaque = true;
                            continue;
                        } else if vfs.is::<Vfs>() {
                            let vfs = vfs
                            .borrow::<Vfs>()
                            .map_err(|_| LuaError::external("Failed to borrow Vfs"))?;

                            vfs_refs.push(mluau_require::vfs::VfsPath::new(vfs.vfs.clone()));
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

            Ok(Vfs { vfs: Arc::new(mluau_require::vfs::OverlayFS::new(&vfs_refs)), from_opaque })
        });

        methods.add_method("createrequirefunction", |lua, this, (id, global_table): (String, LuaTable)| {
            let controller = AssetRequirer::new(FilesystemWrapper::new(this.vfs.clone()), id, global_table);
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
