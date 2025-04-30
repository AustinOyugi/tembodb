use crate::memory::context::TemboMemoryContext;
use crate::memory::context_registry::{
    set_current_context, ContextRegistry, MEMORY_CONTEXT_REGISTRY,
};
use log::debug;
use std::io;
use std::sync::Arc;

/// Memory allocations happen within contexts
/// Every chunk allocated must be withing the bounds of the context
/// That way we can easily free the memory by just clearing the context
pub fn init_memory_base_context() -> io::Result<()> {
    debug!("Initializing memory contexts");

    let top_mem_ctx = TemboMemoryContext::new(
        ContextRegistry::TopLevelMemoryContext.to_string().as_str(),
        None,
    );

    MEMORY_CONTEXT_REGISTRY.write().unwrap().insert(
        ContextRegistry::TopLevelMemoryContext,
        Arc::clone(&top_mem_ctx),
    );

    debug!("Top level memory context initialized");

    let error_mem_ctx = TemboMemoryContext::new(
        ContextRegistry::ErrorMemoryContext.to_string().as_str(),
        Some(top_mem_ctx.clone()),
    );

    MEMORY_CONTEXT_REGISTRY.write().unwrap().insert(
        ContextRegistry::ErrorMemoryContext,
        Arc::clone(&error_mem_ctx),
    );

    debug!("Error memory context initialized");

    set_current_context(top_mem_ctx);

    Ok(())
}
