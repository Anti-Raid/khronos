// Ported from https://github.com/luau-lang/luau/blob/master/CLI/src/VfsNavigator.cpp
use super::fswrapper::FilesystemWrapper;
use super::utils::{is_absolute_path, normalize_path};
use std::path::{Path, PathBuf};

const SUFFIXES: [&str; 2] = [".luau", ".lua"];
const INIT_SUFFIXES: [&str; 2] = ["/init.luau", "/init.lua"];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NavigationStatus {
    Success,
    NotFound,
    Ambiguous,
}

pub struct ResolvedRealPath {
    status: NavigationStatus,
    real_path: Option<String>,
}

pub struct VfsNavigator {
    pub fs: FilesystemWrapper,
    real_path: String,
    absolute_real_path: String,
    absolute_path_prefix: String,
    module_path: String,
    absolute_module_path: String,
}

impl VfsNavigator {
    pub fn new(fs: FilesystemWrapper) -> Self {
        Self {
            fs,
            real_path: "/".to_string(),
            absolute_real_path: "/".to_string(),
            absolute_path_prefix: "".to_string(),
            module_path: "/".to_string(),
            absolute_module_path: "/".to_string(),
        }
    }
}

impl VfsNavigator {
    pub(super) fn get_real_path(
        &self,
        module_path: String,
    ) -> Result<ResolvedRealPath, crate::Error> {
        let mut found = false;
        let mut suffix = "";

        // Get the position of the last slash
        let last_slash = module_path.rfind('/').unwrap_or(0);
        let last_component = if last_slash != 0 {
            &module_path[last_slash + 1..]
        } else {
            ""
        };

        log::trace!("Get_real_path: {module_path}");

        if last_component != "init" {
            for potential_suffix in SUFFIXES.iter() {
                if self
                    .fs
                    .is_file(format!("{module_path}{potential_suffix}"))?
                {
                    if found {
                        return Ok(ResolvedRealPath {
                            status: NavigationStatus::Ambiguous,
                            real_path: None,
                        });
                    }

                    suffix = potential_suffix;
                    found = true;
                }
            }
        }

        if self.fs.is_dir(module_path.clone())? {
            if found {
                return Ok(ResolvedRealPath {
                    status: NavigationStatus::Ambiguous,
                    real_path: None,
                });
            }

            for potential_suffix in INIT_SUFFIXES.iter() {
                if self
                    .fs
                    .is_file(format!("{module_path}{potential_suffix}"))?
                {
                    if found {
                        return Ok(ResolvedRealPath {
                            status: NavigationStatus::Ambiguous,
                            real_path: None,
                        });
                    }

                    suffix = potential_suffix;
                    found = true;
                }
            }

            found = true;
        }

        if !found {
            return Ok(ResolvedRealPath {
                status: NavigationStatus::NotFound,
                real_path: None,
            });
        }

        Ok(ResolvedRealPath {
            status: NavigationStatus::Success,
            real_path: Some(format!("{module_path}{suffix}")),
        })
    }
}

fn get_module_path(file_path: &mut String) -> String {
    // Normalize separators: replace '\\' with '/'
    // Iterate over the bytes of the string and replace '\\' (byte value 92)
    // with '/' (byte value 47).
    *file_path = file_path.replace('\\', "/");

    // Create a string view (slice) from the modified path
    let mut path_view: &str = file_path;

    log::trace!("path_view: {path_view}");

    // Handle absolute paths
    if is_absolute_path(path_view) {
        let first_slash_option = path_view.find('/');

        // Assert that a slash was found.
        assert!(
            first_slash_option.is_some(),
            "Absolute path must contain a slash"
        );

        let first_slash_index = first_slash_option.unwrap();

        path_view = &path_view[first_slash_index..];
    }

    for suffix in INIT_SUFFIXES.iter() {
        if path_view.ends_with(suffix) {
            path_view = &path_view[..path_view.len() - suffix.len()];

            // BUGFIX: Avoid '.' from being a module_path
            if path_view == "." {
                return "".to_string();
            }

            return path_view.to_string();
        }
    }

    // Remove suffixes from kSuffixes
    for suffix in SUFFIXES.iter() {
        if path_view.ends_with(suffix) {
            path_view = &path_view[..path_view.len() - suffix.len()];

            // BUGFIX: Avoid '.' from being a module_path
            if path_view == "." {
                return "".to_string();
            }

            return path_view.to_string();
        }
    }

    // BUGFIX: Avoid '.' from being a module_path
    if path_view == "." {
        return "".to_string();
    }

    path_view.to_string()
}

impl VfsNavigator {
    pub fn update_real_paths(&mut self) -> Result<NavigationStatus, crate::Error> {
        let result = self.get_real_path(self.module_path.clone())?;
        let absolute_result = self.get_real_path(self.absolute_module_path.clone())?;
        if result.status != NavigationStatus::Success
            || absolute_result.status != NavigationStatus::Success
        {
            if self.module_path.is_empty() {
                // DEVIATION: Support rooted init.luau
                log::trace!("Deviation triggered: empty module_path");
            }
            return Ok(result.status);
        }

        assert!(result.real_path.is_some(), "result.real_path is none!");
        assert!(
            absolute_result.real_path.is_some(),
            "result.real_path is none!"
        );
        let result_real_path = result.real_path.unwrap();
        let absolute_result_real_path = absolute_result.real_path.unwrap();
        self.real_path = if is_absolute_path(&result_real_path) {
            format!("{}{}", self.absolute_path_prefix, result_real_path)
        } else {
            result_real_path
        };
        self.absolute_real_path =
            format!("{}{}", self.absolute_path_prefix, absolute_result_real_path);
        Ok(NavigationStatus::Success)
    }

    pub fn reset_to_stdin(&mut self) -> Result<NavigationStatus, crate::Error> {
        self.real_path = "./stdin".to_string();
        self.absolute_real_path = "/stdin".to_string();
        self.module_path = "./stdin".to_string();
        self.absolute_module_path = "/stdin".to_string();
        self.absolute_path_prefix = "".to_string();
        Ok(NavigationStatus::Success)
    }

    pub fn reset_to_path(&mut self, path: &Path) -> Result<NavigationStatus, crate::Error> {
        let mut normalized_path = normalize_path(path).to_string_lossy().to_string();

        log::trace!("Normalized path: {normalized_path}");

        if is_absolute_path(&normalized_path) {
            self.module_path = get_module_path(&mut normalized_path);
            self.absolute_module_path = self.module_path.clone();

            let first_slash = normalized_path.find('/').unwrap_or(0);
            self.absolute_path_prefix = normalized_path[0..first_slash].to_string();
        } else {
            let cwd = "";

            self.module_path = get_module_path(&mut normalized_path);

            let mut joined_path =
                normalize_path(&PathBuf::from(cwd.to_string() + "/" + &normalized_path))
                    .to_string_lossy()
                    .to_string();
            self.absolute_module_path = get_module_path(&mut joined_path);

            let first_slash = joined_path.find('/').unwrap_or(0);

            self.absolute_path_prefix = joined_path[0..first_slash].to_string();
        }

        if self.module_path.is_empty() {
            self.module_path = "/".to_string(); // DEVIATION: Support rooted modules
        }

        if self.absolute_module_path.is_empty() {
            self.absolute_module_path = "/".to_string(); // DEVIATION: Support rooted modules
        }

        log::trace!("module_path: {}", self.module_path);
        self.update_real_paths()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_parent(&mut self) -> Result<NavigationStatus, crate::Error> {
        log::trace!("AbsModPath: {}", self.absolute_module_path);

        if self.absolute_module_path.is_empty() {
            return Ok(NavigationStatus::NotFound);
        }

        // DEVIATION: Allow "" as a parent view to root dir
        if self.absolute_module_path == "/" {
            self.module_path = "".to_string();
            self.absolute_module_path = "".to_string();
            log::trace!("Deviation: set module_path + abs_module_path to empty");
            return self.update_real_paths();
        }

        let mut num_slashes = 0;
        for c in self.absolute_module_path.chars() {
            if c == '/' {
                num_slashes += 1;
            }
        }

        if num_slashes <= 0 {
            return Err("num_slashes <= 0".into());
        }

        if num_slashes == 1 {
            self.module_path = "".to_string();
            self.absolute_module_path = "".to_string();
            return self.update_real_paths();
        }

        self.module_path = normalize_path(&PathBuf::from(self.module_path.clone() + "/.."))
            .to_string_lossy()
            .to_string();
        log::trace!("NewModPath: {}", self.module_path);
        self.absolute_module_path =
            normalize_path(&PathBuf::from(self.absolute_module_path.clone() + "/.."))
                .to_string_lossy()
                .to_string();
        log::trace!("NewAbsModPath: {}", self.absolute_module_path);
        self.update_real_paths()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_child(&mut self, name: &str) -> Result<NavigationStatus, crate::Error> {
        self.module_path = normalize_path(&PathBuf::from(self.module_path.clone() + "/" + name))
            .to_string_lossy()
            .to_string();
        self.absolute_module_path = normalize_path(&PathBuf::from(
            self.absolute_module_path.clone() + "/" + name,
        ))
        .to_string_lossy()
        .to_string();

        self.update_real_paths()
    }

    pub fn get_file_path(&self) -> &str {
        &self.real_path
    }

    pub fn get_absolute_file_path(&self) -> &str {
        &self.absolute_real_path
    }

    pub fn get_luaurc_path(&self) -> String {
        log::trace!("get_luaurc_path called");
        let mut directory = self.real_path.as_str();

        for suffix in INIT_SUFFIXES.iter() {
            if directory.ends_with(suffix) {
                directory = &directory[..directory.len() - suffix.len()];
                return format!("{directory}/.luaurc");
            }
        }
        for suffix in SUFFIXES.iter() {
            if directory.ends_with(suffix) {
                directory = &directory[..directory.len() - suffix.len()];
                return format!("{directory}/.luaurc");
            }
        }
        format!("{directory}/.luaurc")
    }
}
