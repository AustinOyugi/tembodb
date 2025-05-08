use std::collections::BTreeMap;
use crate::catalog::tembo_catalog::TemboRelCatalog;
use crate::catalog::tembo_rel_def::TemboRelDef;
use crate::memory::context::TemboMemoryContext;
use crate::memory::context_registry::CURRENT_MEMORY_CONTEXT;
use std::{io, process};
use std::io::Error;
use std::sync::{Arc, Mutex, RwLock};
use log::error;

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
            }
        }
    }
}


/// At this stage we want to initialize the base tembo utility classes that
/// load all the systems metadata in memory, this is in preparation the 
/// context switch to the normal mode after. 
/// Loads:
/// Tembo Rel Def
/// Tembo Rel Attributes
/// Tembo Rel Type
pub fn initialize_rel_descriptors() -> io::Result<()> {

    let bootstrap_context = CURRENT_MEMORY_CONTEXT
        .with( |ctx| ctx.borrow().clone());

    let bootstrap_state = BootstrapState::new(bootstrap_context.clone());

    let tembo_rel_def_class =  TemboRelDef::bootstrap(bootstrap_context);

    match bootstrap_state.catalog.register_class(tembo_rel_def_class) {
        Ok(_) => {}
        Err(err) => {
            error!("Error registering tembo class to catalog! {}", err);
            return  Err(Error::new(io::ErrorKind::Other,err));
        }
    };
    
    Ok(())
}
