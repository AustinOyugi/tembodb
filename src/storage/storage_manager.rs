use std::fs;
use std::path::Path;
use log::{debug, info, trace};
use crate::config::base_configs::BaseConfig;

pub fn is_storage_ready(base_config:  &BaseConfig) -> bool {
    let base_directory = &base_config.file_path;
    vec!["/storage", "/base","/global"].into_iter().all(|path| {
        let full_path:  String = format!("{base_directory}{path}");
        trace!("Checking if path {} exists", full_path);
        let exists = Path::new(&full_path).exists();
        trace!("Path {} {}", full_path, exists);
        exists
    })
}
