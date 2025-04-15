mod config;

use config::base_configs::BaseConfig;

fn main() -> std::io::Result<()> {
    let base_configs = BaseConfig::load();
    println!("{:?}", base_configs);
    Ok(())
}
