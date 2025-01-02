#![allow(unused_must_use)]

use cache_dir::{get_cache_dir, get_data_dir, NoSuchDirectoryError};

#[cfg_attr(target_os = "android", mobile_entry_point::mobile_entry_point)]
fn main() -> Result<(), NoSuchDirectoryError> {
    println!("Cache directory: {}", get_cache_dir()?.display());
    println!("Writable data directory: {}", get_data_dir()?.display());

    Ok(())
}
