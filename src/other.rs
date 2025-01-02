use std::path::PathBuf;
use dirs::data_local_dir;

pub(super) fn cache_dir() -> Option<PathBuf> {
    data_local_dir().map(|p| p.join("Temp"))
}

pub(super) fn data_dir() -> Option<PathBuf> {
    data_local_dir()
}