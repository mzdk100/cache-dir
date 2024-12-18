use cache_dir::{get_cache_dir, NoSuchDirectoryError};

#[cfg_attr(target_os = "android", mobile_entry_point::mobile_entry_point)]
fn main() -> Result<(), NoSuchDirectoryError> {
    println!("{}", get_cache_dir()?.display());
    Ok(())
}
