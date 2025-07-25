use std::collections::HashMap;
use std::io::Read;
use bstr::BString;
use mluau::prelude::*;
use bstr::ByteSlice;

use crate::primitives::blob::{Blob, BlobTaker};

pub struct TarArchive {
    pub entries: HashMap<BString, Blob>,
}

impl TarArchive {
    /// Makes a empty tar archive
    pub fn new() -> Self {
        TarArchive {
            entries: HashMap::new(),
        }
    }

    /// Adds an entry to the tar archive
    pub fn add_entry(&mut self, name: LuaString, blob: BlobTaker) {
        self.entries.insert(BString::new(name.as_bytes().to_vec()), Blob { data: blob.0 });
    }

    /// Takes an entry by name, removing it from the archive
    pub fn take_entry(&mut self, name: &str) -> Option<Blob> {
        self.entries.remove(&BString::from(name))
    }

    /// Given a Blob, attempts to read it as a tar archive
    pub fn from_blob(blob: Blob) -> LuaResult<Self> {
        Self::from_array(blob.data)
    }

    pub fn from_array(arr: Vec<u8>) -> LuaResult<Self> {
        let mut entries = HashMap::new();
        let mut archive = tar::Archive::new(arr.as_slice());

        for entry in archive.entries()? {
            let mut entry = entry?;
            let header = entry.header();
            // Convert the path to a byte string
            let path = header.path_bytes();
            let path_bstr = BString::from(path.as_ref());

            // Read the entry data into a Blob
            let mut data = Vec::new();
            entry.read_to_end(&mut data)?;

            entries.insert(path_bstr, Blob { data });
        }

        Ok(TarArchive { entries })
    }

    /// Writes the tar archive to a Blob
    pub fn to_blob(self) -> LuaResult<Blob> {
        let mut buffer = Vec::new();
        {
            let mut tar = tar::Builder::new(&mut buffer);
            for (path, blob) in self.entries {
                let mut header = tar::Header::new_gnu();
                header.set_size(blob.data.len() as u64);
                tar.append_data(
                    &mut header,
                    path.to_path_lossy(),
                    blob.data.as_slice(),
                )?;
            }
            tar.finish()?;
        }

        Ok(Blob { data: buffer })
    }
}

impl LuaUserData for TarArchive {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Len, |_, this, ()| {
            Ok(this.entries.len())
        });

        methods.add_method_mut("take_entry", |lua, this, name: String| {
            if let Some(blob) = this.take_entry(&name) {
                let blob = blob.into_lua(lua)?;
                Ok(blob)
            } else {
                Ok(LuaNil)
            }
        });

        methods.add_method_mut("add_entry", |_, this, (name, blob): (LuaString, BlobTaker)| {
            this.add_entry(name, blob);
            Ok(())
        });

        methods.add_function("blob", |_, this: LuaAnyUserData| {
            let this = this.take::<Self>()?;
            this.to_blob()
        });

        methods.add_method("entries", |lua, this, ()| {
            let mut entries = Vec::with_capacity(this.entries.len());
            for section in this.entries.keys() {
                entries.push(lua.create_string(section)?);   
            }
            Ok(entries)
        });
    }

    #[cfg(feature = "repl")]
    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
    }
}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set("newblob", lua.create_function(|_, buf: mluau::Buffer| {
        Ok(Blob {
            data: buf.to_vec(),
        })
    })?)?;

    module.set("TarArchive", lua.create_function(|_, blob: Option<BlobTaker>| {
        if let Some(blob) = blob {
            TarArchive::from_array(blob.0).map_err(LuaError::external)
        } else {
            Ok(TarArchive::new())
        }
    })?)?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tar_archive() {
        let mut archive = TarArchive::new();
        archive.entries.insert(
            BString::from("foo/test.txt"),
            Blob {
                data: b"Hello, world!".to_vec(),
            },
        );
        let blob = archive.to_blob().unwrap();
        let mut tar_archive = TarArchive::from_blob(blob).expect("Failed to read tar archive");
        assert_eq!(tar_archive.take_entry("foo/test.txt").unwrap().data, b"Hello, world!".as_bytes());
    }
}