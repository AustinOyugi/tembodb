use super::config_manager;
use log::error;
use std::fmt::Formatter;
use std::{fmt, process};

#[derive(Debug)]
pub struct BaseConfig {
    pub file_path: String,
    pub first_bootstrap_oid: u32,
    pub last_bootstrap_oid: u32,
}

impl fmt::Display for BaseConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl BaseConfig {
    pub fn load() -> Self {
        let config_values = config_manager::extract_value_mapper();
        BaseConfig {
            file_path: un_wrapper("file_path", config_values.get("file_path")),
            first_bootstrap_oid: 1,
            last_bootstrap_oid: 1,
        }
    }
}

fn un_wrapper(key: &str, option: Option<&String>) -> String {
    match option {
        None => {
            error!("Error loading configuration key!! {}", key);
            process::exit(1)
        }
        Some(value) => value.parse().unwrap(),
    }
}
