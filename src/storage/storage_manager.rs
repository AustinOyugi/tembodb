use crate::config::base_configs::BaseConfig;
use crate::constants::constants::BASE_CONFIGS;
use log::trace;
use std::path::Path;
use std::{fs, io};

fn get_base_dirs() -> Vec<&'static str> {
    vec!["/storage", "/base", "/global"]
}

pub fn is_storage_ready() -> bool {
    let base_directory = &BASE_CONFIGS.get().unwrap().file_path;
    get_base_dirs().into_iter().all(|path| {
        let full_path: String = format!("{base_directory}{path}");
        trace!("Checking if path {} exists", full_path);
        let exists = Path::new(&full_path).exists();
        trace!("Path {} {}", full_path, exists);
        exists
    })
}

pub fn initialize_storage_dirs() -> io::Result<()> {
    let base_directory = &BASE_CONFIGS.get().unwrap().file_path;
    for path in get_base_dirs().iter() {
        trace!("Creating dir {}", path);
        let full_path: String = format!("{base_directory}{path}");
        fs::create_dir_all(full_path)?;
    }
    Ok(())
}
