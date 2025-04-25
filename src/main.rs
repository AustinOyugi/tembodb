mod config;
mod environment;
mod storage;
mod memory;

use config::base_configs::BaseConfig;
use environment::initialize_env::initialize_environment;
use environment::initialize_env::validate_env_ready;
use log::info;

fn main() -> std::io::Result<()> {

    // Init Db
    // Enter bootstrap mode (--boot)
    // Initialize memory, pages, catalogs
    // Write raw catalog rows using Rust structs
    // Build indexes and system views
    // Set up template0, template1, tembo
    // Finalize WAL and control files
    // Ready for client connections
    

    // Memory management (context system)
    // 
    // Basic utilities (error handling, logging)
    // 
    // Storage layer (raw page access)
    // 
    // System catalogs (tembo_class)

    env_logger::init();
    info!("Stating Tembo DB.");
    let base_configs = BaseConfig::load();
    initialize_environment(&base_configs)?;

    if validate_env_ready(&base_configs) {
        info!("Environment ... ok");
    }

    // // Phase 1: Memory and basic infrastructure
    // MemoryContextInit();

    // CrashRecoveryInit();
    // 
    // // Phase 2: Raw storage initialization
    // StorageManagerInit();
    // 
    // // Phase 3: Bootstrap catalogs
    // TemboClassBootstrapInit();  // Critical initialization
    // 
    // // Phase 4: Normal database operation
    // TransactionSystemInit();
    Ok(())
}
