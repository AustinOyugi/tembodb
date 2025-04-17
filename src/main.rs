mod config;
mod environment;
mod storage;

use config::base_configs::BaseConfig;
use environment::initialize_env::initialize_environment;
use environment::initialize_env::validate_env_ready;
use log::info;

fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Stating Tembo DB.");
    let base_configs = BaseConfig::load();
    if validate_env_ready(&base_configs) {
        info!("Environment ... ok");
    } else {
        info!("Environment not ready!! proceeding with initialization");
        initialize_environment(&base_configs)?;
    }
    Ok(())
}
