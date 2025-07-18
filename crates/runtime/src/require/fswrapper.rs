use std::rc::Rc;
use vfs::path::VfsFileType;
use vfs::{FileSystem, VfsResult};

#[derive(Debug, Clone)]
/// A wrapper around a VFS file system
pub struct FilesystemWrapper(pub Rc<dyn FileSystem>);

impl FilesystemWrapper {
    pub fn new<T: vfs::FileSystem>(fs: T) -> Self {
        Self(Rc::new(fs))
    }

    pub fn read_file(&self, path: &str) -> VfsResult<Vec<u8>> {
        self.read_to_bytes(path)
    }

    /// Fixes the path to conform to the VFS specific quirks/format
    pub fn path_fix(path: String) -> String {
        if path.starts_with("./") {
            return format!("/{}", path.trim_start_matches("./"));
        } else if !path.starts_with('/') {
            return format!("/{path}");
        }

        path
    }

    pub fn is_file(&self, path: String) -> VfsResult<bool> {
        let path = Self::path_fix(path);

        log::trace!("Checking if {path:#?} is a file");
        if !self.exists(&path)? {
            log::trace!("File {path:#?} does not exist");
            return Ok(false);
        }

        let metadata = self.metadata(&path)?;
        Ok(metadata.file_type == VfsFileType::File)
    }

    pub fn get_file(&self, path: String) -> VfsResult<Vec<u8>> {
        let path = Self::path_fix(path);

        let contents = self.read_file(&path)?;
        Ok(contents)
    }

    pub fn is_dir(&self, path: String) -> VfsResult<bool> {
        let path = Self::path_fix(path);

        log::trace!("Checking if {path:#?} is a directory");
        if path.is_empty() || path == "/" {
            return Ok(true);
        }

        if !self.exists(&path)? {
            log::trace!("Directory {path:#?} does not exist");
            return Ok(false);
        }

        let metadata = self.0.metadata(&path)?;
        log::trace!("Metadata: {metadata:#?}");
        Ok(metadata.file_type == VfsFileType::Directory)
    }
}

impl std::ops::Deref for FilesystemWrapper {
    type Target = dyn FileSystem;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
