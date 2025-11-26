use crate::require::FilesystemWrapper;
use std::collections::{HashMap, HashSet};
use vfs::FileSystem;

/// Creates a virtual filesystem from a map of paths to content.
pub fn create_vfs_from_map(
    tree: &HashMap<String, String>,
) -> Result<FilesystemWrapper, crate::Error> {
    let fs = FilesystemWrapper::new(vfs::MemoryFS::new());
    let mut created = HashSet::new();
    for (path_s, content) in tree {
        let path = path_s.split('/').collect::<Vec<_>>();
        if path.len() >= 2 {
            // Folder part is everything except the last part
            let mut folder_part = Vec::with_capacity(path.len() - 1);
            let plen = path.len();
            for (i, part) in path.into_iter().enumerate() {
                if i == plen - 1 {
                    break;
                }
                folder_part.push(part);
            }

            let mut current_path = String::new();
            for folder in folder_part {
                current_path.push('/');
                current_path.push_str(folder);
                if !created.contains(&current_path) {
                    fs.create_dir(&current_path)?;
                    created.insert(current_path.clone());
                }
            }
        }
        let path_s = format!("/{path_s}");
        fs.create_file(&path_s)
            .map_err(|e| crate::Error::from(format!("Failed to create file {path_s}: {e}")))?
            .write_all(content.as_bytes())
            .map_err(|e| crate::Error::from(format!("Failed to write to {path_s}: {e}")))?;
    }

    Ok(fs)
}

/// Creates a virtual filesystem that is a concrete MemoryFS from a map of paths to content.
pub fn create_memory_vfs_from_map(
    tree: std::collections::HashMap<String, String>,
) -> Result<vfs::MemoryFS, crate::Error> {
    let fs = vfs::MemoryFS::new();
    let mut created = HashSet::new();
    for (path_s, content) in tree {
        let path = path_s.split('/').collect::<Vec<_>>();
        if path.len() >= 2 {
            // Folder part is everything except the last part
            let mut folder_part = Vec::with_capacity(path.len() - 1);
            let plen = path.len();
            for (i, part) in path.into_iter().enumerate() {
                if i == plen - 1 {
                    break;
                }
                folder_part.push(part);
            }

            let mut current_path = String::new();
            for folder in folder_part {
                current_path.push('/');
                current_path.push_str(folder);
                if !created.contains(&current_path) {
                    fs.create_dir(&current_path)?;
                    created.insert(current_path.clone());
                }
            }
        }
        let path_s = format!("/{path_s}");
        fs.create_file(&path_s)
            .map_err(|e| crate::Error::from(format!("Failed to create file {path_s}: {e}")))?
            .write_all(content.as_bytes())
            .map_err(|e| crate::Error::from(format!("Failed to write to {path_s}: {e}")))?;
    }

    Ok(fs)
}
