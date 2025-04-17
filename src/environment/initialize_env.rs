use crate::config::base_configs::BaseConfig;
use crate::storage::storage_manager::is_storage_ready;

fn get_env_ready_func_registry() -> Vec<fn(base_config:  &BaseConfig) -> bool> {
    vec![is_storage_ready]
}

pub fn validate_env_ready(base_config:  &BaseConfig) -> bool{
    // Storage manager to check if ready
    for ready_fb in  get_env_ready_func_registry().into_iter() {
        if !ready_fb(base_config) {
            false;
        }
    }
    true
}
