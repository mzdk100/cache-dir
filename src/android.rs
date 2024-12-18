use droid_wrap::android::app::Activity;
use std::path::PathBuf;

pub(super) fn cache_dir() -> Option<PathBuf> {
    Some(PathBuf::from(
        &Activity::fetch().get_cache_dir().to_string(),
    ))
}