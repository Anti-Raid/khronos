use std::collections::HashMap;
use std::io::Read;
use bstr::BString;
use mluau::prelude::*;
use bstr::ByteSlice;

use crate::primitives::blob::Blob;

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

    /// Reads a entry by name 
    pub fn get_entry(&self, name: &str) -> Option<&Blob> {
        self.entries.get(&BString::from(name))
    }

    /// Given a Blob, attempts to read it as a tar archive
    pub fn from_blob(blob: Blob) -> LuaResult<Self> {
        let mut entries = HashMap::new();
        let mut archive = tar::Archive::new(blob.data.as_slice());

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

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    // Null
    module.set("newblob", lua.create_function(|_, buf: mluau::Buffer| {
        Ok(Blob {
            data: buf.to_vec(),
        })
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
        let tar_archive = TarArchive::from_blob(blob).expect("Failed to read tar archive");
        assert_eq!(tar_archive.get_entry("foo/test.txt").unwrap().data, b"Hello, world!".as_bytes());
    }
}