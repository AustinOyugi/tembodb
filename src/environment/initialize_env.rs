use crate::config::base_configs::BaseConfig;
use crate::storage::storage_manager::{initialize_storage_dirs, is_storage_ready};
use log::{error, trace};
use std::{io, process};

fn get_env_ready_func_registry() -> Vec<fn(base_config: &BaseConfig) -> bool> {
    vec![is_storage_ready]
}

fn get_env_init_func_registry() -> Vec<fn(base_config: &BaseConfig) -> io::Result<()>> {
    vec![initialize_storage_dirs]
}

pub fn validate_env_ready(base_config: &BaseConfig) -> bool {
    // Storage manager to check if ready
    for ready_fn in get_env_ready_func_registry().into_iter() {
        let is_ready = ready_fn(base_config);
        if is_ready == false {
            return false;
        }
    }
    true
}

fn get_function_name<F, Args, Output>(_: F) -> &'static str
where
    F: Fn(Args) -> Output,
{
    std::any::type_name::<F>()
}

pub fn initialize_environment(base_config: &BaseConfig) -> io::Result<()> {
    for init_fn in get_env_init_func_registry().into_iter() {
        match init_fn(base_config) {
            Ok(_) => {
                trace!(
                    "Feature {} successfully initialized",
                    get_function_name(init_fn)
                )
            }
            Err(err) => {
                error!(
                    "Feature {} failed to be initialized, {}",
                    get_function_name(init_fn),
                    err
                );
                process::exit(1)
            }
        }
    }
    Ok(())
}
