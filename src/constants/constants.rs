use crate::config::base_configs::BaseConfig;
use lazy_static::lazy_static;
use std::sync::{OnceLock, RwLock};

lazy_static! {
    pub static ref BOOTSTRAP_MODE_ACTIVE: RwLock<bool> = RwLock::new(false);
}

lazy_static! {
    // Makes sure the base configs are only set once and cannot be mutated
     pub static ref BASE_CONFIGS: OnceLock<BaseConfig> = OnceLock::new();
}
