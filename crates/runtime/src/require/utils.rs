use std::collections::VecDeque;
use std::path::{Component, Path, PathBuf};

// From https://github.com/luau-lang/luau/blob/master/CLI/src/FileUtils.cpp#L49
pub(super) fn is_absolute_path(path: &str) -> bool {
    #[cfg(windows)]
    {
        // Must either begin with "X:/", "X:\", "/", or "\", where X is a drive letter
        (path.len() >= 3
            && path.as_bytes()[0].is_ascii_alphabetic()
            && path.as_bytes()[1] == b':'
            && (path.as_bytes()[2] == b'/' || path.as_bytes()[2] == b'\\'))
            || (path.len() >= 1 && (path.as_bytes()[0] == b'/' || path.as_bytes()[0] == b'\\'))
    }
    #[cfg(not(windows))]
    {
        // Must begin with '/'
        !path.starts_with('/')
    }
}

// Normalizes the path by removing unnecessary components
pub(super) fn normalize_path(path: &Path) -> PathBuf {
    let mut components = VecDeque::new();

    for comp in path.components() {
        match comp {
            Component::Prefix(..) | Component::RootDir => {
                components.push_back(comp);
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if matches!(components.back(), None | Some(Component::ParentDir)) {
                    components.push_back(Component::ParentDir);
                } else if matches!(components.back(), Some(Component::Normal(..))) {
                    components.pop_back();
                }
            }
            Component::Normal(..) => components.push_back(comp),
        }
    }

    if matches!(components.front(), None | Some(Component::Normal(..))) {
        components.push_front(Component::CurDir);
    }

    // Join the components back together
    components.into_iter().collect()
}
