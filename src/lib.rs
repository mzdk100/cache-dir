//! 跨平台获取缓存目录的函数

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "ios")]
mod ios;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod other;

#[cfg(target_os = "android")]
use android::{cache_dir, data_dir};
#[cfg(target_os = "ios")]
use ios::{cache_dir, data_dir};
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use other::{cache_dir, data_dir};

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    path::PathBuf,
};

/// 自定义错误类型，表示在当前平台上找不到缓存目录的情况
#[derive(Debug)]
pub struct NoSuchDirectoryError;

impl Display for NoSuchDirectoryError {
    /// 格式化错误信息
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Can't find the cache directory on current platform.")
    }
}

impl Error for NoSuchDirectoryError {}

/// 获取缓存目录的函数。
/// 缓存目录是一种非常不可控的目录类型，目录中的文件什么时候会被删除完全取决于操作系统，特别是在移动设备上，如果您的APP使用了较多的空间，当用户在使用其他APP时，可能优先删除您APP中的数据，在这种情况下可以改用[get_data_dir]来代替。
/// 在Windows上，此函数获取的是`%LOCALAPPDATA%\Temp`，其他平台取决于操作系统或者[dirs::data_local_dir]的实现；
///
/// 尝试获取当前平台的缓存目录。如果成功，返回一个包含目录路径的`Ok`值；如果失败，返回一个`Err`值，其中包含[NoSuchDirectoryError]错误。
/// 平台包括： Windows, Linux, MacOS, Android, iOS, WASM等。
///
/// # 返回值
///
/// - `Result<PathBuf, NoSuchDirectoryError>`: 成功时返回缓存目录的路径，失败时返回错误信息。
///
/// # 示例
///
/// ```rust
/// use cache_dir::get_cache_dir;
///
/// match get_cache_dir() {
///     Ok(cache_dir) => println!("Cache directory: {:?}", cache_dir),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn get_cache_dir() -> Result<PathBuf, NoSuchDirectoryError> {
    cache_dir().map_or(Err(NoSuchDirectoryError), Ok)
}

/// 获取数据目录的函数。
/// 数据目录是一种可靠的存储目录，可随时读写文件且无需申请权限，与[get_cache_dir]的区别是，此函数获取的数据目录更加持久。
/// 在安卓平台此目录是APP私有目录中的`files`，Windows上是`%LOCALAPPDATA%`，其他平台取决于操作系统或者[dirs::data_local_dir]的实现；
///
/// 尝试获取当前平台的可写的数据目录。如果成功，返回一个包含目录路径的`Ok`值；如果失败，返回一个`Err`值，其中包含[NoSuchDirectoryError]错误。
/// 平台包括： Windows, Linux, MacOS, Android, iOS, WASM等。
///
/// # 返回值
///
/// - `Result<PathBuf, NoSuchDirectoryError>`: 成功时返回数据目录的路径，失败时返回错误信息。
///
/// # 示例
///
/// ```rust
/// use cache_dir::get_data_dir;
///
/// match get_data_dir() {
///     Ok(data_dir) => println!("Writable data directory: {:?}", data_dir),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn get_data_dir() -> Result<PathBuf, NoSuchDirectoryError> {
    data_dir().map_or(Err(NoSuchDirectoryError), Ok)
}
