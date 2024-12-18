use objc2::{class, msg_send, runtime::AnyObject};
use objc2_foundation::{NSArray, NSObject, NSString};
use std::{ffi::c_ulong, path::PathBuf};

const NS_USER_DOMAIN_MASK: c_ulong = 1;

enum AppleDirType {
    Library,
    User,
    Document,
    Cache,
    ApplicationSupport,
    Downloads,
}

impl Into<c_ulong> for AppleDirType {
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
    unsafe {
        let ns_file_manager = class!(NSFileManager);
        let instance: *mut AnyObject = msg_send![ns_file_manager, defaultManager];
        let directories: *const NSArray<NSObject> =
            msg_send![instance, URLsForDirectory:AppleDirType::Cache inDomains:NS_USER_DOMAIN_MASK];
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
