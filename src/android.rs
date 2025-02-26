use droid_wrap::android::app::Activity;
use std::path::PathBuf;

pub(super) fn cache_dir() -> Option<PathBuf> {
    let Ok(act) = Activity::fetch() else {
        return None;
    };

    Some(PathBuf::from(&act.get_cache_dir().to_string()))
}

pub(super) fn data_dir() -> Option<PathBuf> {
    let Ok(act) = Activity::fetch() else {
        return None;
    };

    Some(PathBuf::from(&act.get_files_dir().to_string()))
}
