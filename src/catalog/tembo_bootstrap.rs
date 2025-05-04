use std::io;
use std::sync::RwLock;
use lazy_static::lazy_static;
use crate::catalog::tembo_rel_def::TemboRelDef;
use crate::config::base_configs::BaseConfig;
use crate::memory::context_registry::CURRENT_MEMORY_CONTEXT;

pub fn initialize_rel_descriptors() -> io::Result<()> {

    CURRENT_MEMORY_CONTEXT.with(|ctx| {
            TemboRelDef::bootstrap(ctx.borrow().clone())
    });

    Ok(())
}
