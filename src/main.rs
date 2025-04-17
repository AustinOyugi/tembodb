mod config;
mod environment;
mod storage;

use log::info;
use config::base_configs::BaseConfig;
use environment::initialize_env::validate_env_ready;

fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Stating Tembo DB.");
    let base_configs = BaseConfig::load();
    if !validate_env_ready(&base_configs) {
        info!("Environment not ready!! proceeding with initialization");
    } else {
        info!("Environment ... ok");
    }
    Ok(())
}
