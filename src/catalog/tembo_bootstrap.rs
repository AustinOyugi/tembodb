use std::collections::BTreeMap;
use crate::catalog::tembo_catalog::TemboRelCatalog;
use crate::catalog::tembo_rel_def::TemboRelDef;
use crate::memory::context::TemboMemoryContext;
use crate::memory::context_registry::CURRENT_MEMORY_CONTEXT;
use std::io;
use std::sync::{Arc, Mutex, RwLock};

pub struct BootstrapState {
    context: Arc<Mutex<TemboMemoryContext>>,
    catalog: TemboRelCatalog
}

impl BootstrapState {
    pub fn new(context: Arc<Mutex<TemboMemoryContext>>) -> Self {
        Self {
            context: context.clone(),
            catalog: TemboRelCatalog {
                classes: RwLock::new(BTreeMap::new())
            },
        }
    }
}

pub fn initialize_rel_descriptors() -> io::Result<()> {

    CURRENT_MEMORY_CONTEXT.with(|ctx| {
            TemboRelDef::bootstrap(ctx.borrow().clone())
    });

    Ok(())
}
