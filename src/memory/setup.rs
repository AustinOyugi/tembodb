use std::cell::RefCell;
use crate::memory::context::TemboMemoryContext;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex, RwLock};
use log::{debug, info};
use crate::config::base_configs::BaseConfig;

lazy_static!(
    static ref MEMORY_CONTEXT_REGISTRY: RwLock<HashMap<String,Arc<Mutex<TemboMemoryContext>>>> =
    RwLock::new(HashMap::new());
);

thread_local! {
    static  CURRENT_MEMORY_CONTEXT: RefCell<Arc<Mutex<TemboMemoryContext>>> =
    RefCell::new(TemboMemoryContext::new("TempEmpty",None));
}

/// Memory allocations happen within contexts
/// Every chunk allocated must be withing the bounds of the context
/// That way we can easily free the memory by just clearing the context
pub  fn init_memory_base_context(base_config: &BaseConfig) -> io::Result<()> {

    info!("Initializing memory contexts");

    let top_mem_ctx =
        TemboMemoryContext::new(
            "TopLevelMemoryContext", None);

    MEMORY_CONTEXT_REGISTRY
        .write()
        .unwrap()
        .insert("TopLevelMemoryContext".into(),
                Arc::clone(&top_mem_ctx));

    debug!("Top level memory context initialized");

    let error_mem_ctx =
        TemboMemoryContext::new(
            "ErrorMemoryContext", Some(top_mem_ctx.clone()));

    MEMORY_CONTEXT_REGISTRY
        .write()
        .unwrap()
        .insert("ErrorMemoryContext".into(),
                Arc::clone(&error_mem_ctx));

    debug!("Error memory context initialized");

    CURRENT_MEMORY_CONTEXT.with_borrow_mut(|ctx| {
        *ctx = top_mem_ctx.clone()
    });

    Ok(())
}
