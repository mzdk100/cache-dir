use objc2::{class, msg_send, runtime::AnyObject};
use objc2_foundation::{NSArray, NSObject, NSString};
use std::path::PathBuf;

const NS_USER_DOMAIN_MASK: c_ulong = 1;

#[allow(unused)]
enum AppleDirType {
    Library,
    User,
    Document,
    Cache,
    ApplicationSupport,
    Downloads,
}

impl Into<usize> for AppleDirType {
    fn into(self) -> c_ulong {
        match self {
            Self::Library => 5,
            Self::User => 7,
            Self::Document => 9,
            Self::Cache => 13,
            Self::ApplicationSupport => 14,
            Self::Downloads => 15,
        }
    }
}

pub(super) fn cache_dir() -> Option<PathBuf> {
    let cache_dir: usize = AppleDirType::Cache.into();

    unsafe {
        let ns_file_manager = class!(NSFileManager);
        let instance: *mut AnyObject = msg_send![ns_file_manager, defaultManager];
        let directories: *const NSArray<NSObject> =
            msg_send![instance, URLsForDirectory: cache_dir, inDomains:NS_USER_DOMAIN_MASK];
        if let Some(obj) = (*directories).firstObject() {
            let str: *const NSString = msg_send![&obj, path];
            if str.is_null() {
                return None;
            }
            let copy = (*str).to_string();
            Some(PathBuf::from(copy))
        } else {
            None
        }
    }
}

pub(super) fn data_dir() -> Option<PathBuf> {
    let app_dir: usize = AppleDirType::ApplicationSupport.into();

    unsafe {
        let ns_file_manager = class!(NSFileManager);
        let instance: *mut AnyObject = msg_send![ns_file_manager, defaultManager];
        let directories: *const NSArray<NSObject> =
            msg_send![instance, URLsForDirectory: app_dir, inDomains:NS_USER_DOMAIN_MASK];
        if let Some(obj) = (*directories).firstObject() {
            let str: *const NSString = msg_send![&obj, path];
            if str.is_null() {
                return None;
            }
            let copy = (*str).to_string();
            Some(PathBuf::from(copy))
        } else {
            None
        }
    }
}
