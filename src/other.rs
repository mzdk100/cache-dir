use dirs::data_local_dir;
use std::{fs::create_dir, path::PathBuf};

pub(super) fn cache_dir() -> Option<PathBuf> {
    data_local_dir().map(|p| p.join("Temp")).and_then(|p| {
        if p.exists() {
            Some(p)
        } else {
            create_dir(p.clone()).map(|_| p).ok()
        }
    })
}

pub(super) fn data_dir() -> Option<PathBuf> {
    data_local_dir()
}
