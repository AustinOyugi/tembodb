mod catalog;
mod config;
mod constants;
mod environment;
mod memory;
mod storage;

use crate::constants::constants::BASE_CONFIGS;
use crate::memory::setup::init_memory_base_context;
use config::base_configs::BaseConfig;
use environment::initialize_env::initialize_environment;
use environment::initialize_env::validate_env_ready;
use log::{error, info};
use std::process;

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

    info!("##########   Stating Tembo DB   ##########");

    match BASE_CONFIGS.set(BaseConfig::load()) {
        Ok(_) => {
            info!("Base configurations loaded successfully")
        }
        Err(err) => {
            error!("Error loading configs {}", err);
            process::exit(1)
        }
    }

    init_memory_base_context()?;

    if validate_env_ready() {
        info!("Environment ... ok proceeding with boot");
    } else {
        info!("Environment not ready, entering bootstrap mode");
        initialize_environment()?;
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
