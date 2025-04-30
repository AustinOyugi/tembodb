use crate::memory::context::TemboMemoryContext;
use lazy_static::lazy_static;
use log::trace;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::panic::AssertUnwindSafe;
use std::sync::{Arc, Mutex, RwLock};
use std::{fmt, panic};
use std::ops::Deref;

lazy_static! {
    pub static ref MEMORY_CONTEXT_REGISTRY: RwLock<HashMap<ContextRegistry, Arc<Mutex<TemboMemoryContext>>>> =
        RwLock::new(HashMap::new());
}

thread_local! {
    pub static  CURRENT_MEMORY_CONTEXT: RefCell<Arc<Mutex<TemboMemoryContext>>> =
    RefCell::new(TemboMemoryContext::new(&ContextRegistry::TempEmpty.to_string(),None));
}

/// Contains all the globally available contexts
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ContextRegistry {
    TempEmpty,
    TopLevelMemoryContext,
    ErrorMemoryContext,
    BootstrapContext,
    CatalogMemoryContext,
}

/// Makes sure we have the .to_string()
impl fmt::Display for ContextRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Helps to trance context retrieval
pub fn get_from_context_registry(
    context_name: ContextRegistry,
) -> Option<Arc<Mutex<TemboMemoryContext>>> {
    match MEMORY_CONTEXT_REGISTRY
        .read()
        .unwrap()
        .get(&ContextRegistry::TopLevelMemoryContext)
    {
        None => {
            trace!("Context {:?} not found", context_name);
            None
        }
        Some(cx) => {
            trace!("Context {:?}  found", context_name);
            Some(cx.clone())
        }
    }
}

/// Owns the new context and updates the current thread local context
pub fn set_current_context(new_context: Arc<Mutex<TemboMemoryContext>>) {
    CURRENT_MEMORY_CONTEXT.with_borrow_mut(|ctx| *ctx = new_context.clone());
}

pub fn switch_to<F, T>(new_mem_ctx: Arc<Mutex<TemboMemoryContext>>, f: F) -> T
where
    F: FnOnce() -> T,
{
    CURRENT_MEMORY_CONTEXT.with(|cmc| {
        // Get the previous context
        let previous = cmc.borrow().clone();

        // Set the new context as the current context
        cmc.replace(new_mem_ctx.clone());

        // Execute the closure
        let result = panic::catch_unwind(AssertUnwindSafe(|| f()));

        //Replace the current context with the new context
        cmc.replace(previous);

        match result {
            Ok(val) => val,
            Err(err) => panic::resume_unwind(err), // propagate panic
        }
    })
}
