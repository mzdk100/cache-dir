//! 跨平台获取缓存目录的函数

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "android")]
use android::cache_dir;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use dirs::cache_dir;
#[cfg(target_os = "ios")]
use ios::cache_dir;
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

/// 获取缓存目录的函数
///
/// 尝试获取当前平台的缓存目录。如果成功，返回一个包含目录路径的`Ok`值；如果失败，返回一个`Err`值，其中包含`NoSuchDirectoryError`错误。
/// 平台包括： Windows, Linux, MacOS, iOS, WASM等。
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
    cache_dir().map_or(Err(NoSuchDirectoryError), |path| Ok(path))
}
