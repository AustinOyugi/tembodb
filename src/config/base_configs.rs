use super::config_manager;
use std::process;

#[derive(Debug)]
pub struct BaseConfig {
    pub file_path: String
}

impl BaseConfig {
    pub  fn load () -> Self {
        let  config_values =
            config_manager::extract_value_mapper();
        BaseConfig{
            file_path:  un_wrapper(
                "file_path",
                config_values.get("file_path")),
        }
    }
}

fn un_wrapper(key: &str, option: Option<&String>) -> String{
    match option {
        None => {
            eprintln!("Error loading configuration key!! {}", key);
            process::exit(1)
        }
        Some(value) => {
            value.parse().unwrap()
        }
    }
}
