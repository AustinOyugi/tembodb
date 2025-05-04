use crate::catalog::tembo_rel_def::TemboRelDef;
use crate::constants::constants::{BASE_CONFIGS, BOOTSTRAP_MODE_ACTIVE};
use std::collections::BTreeMap;
use std::io;
use std::io::Error;
use std::sync::{Arc, RwLock};

pub struct TemboRelCatalog {
    pub classes: RwLock<BTreeMap<u32,Arc<TemboRelDef>>>
}

impl TemboRelCatalog {

    pub fn register_class(&self, class: Arc<TemboRelDef>) -> std::io::Result<()>{

        let base_configs = BASE_CONFIGS.get().unwrap();

        if *BOOTSTRAP_MODE_ACTIVE.read().unwrap() {
            if class.obj_id < base_configs.first_bootstrap_oid ||
                class.obj_id > base_configs.last_bootstrap_oid{
                return Err(Error::new(io::ErrorKind::Other,"Invalid bootstrap oid"));
            }
        }

        let mut classes_map = self.classes.write().unwrap();
        if classes_map.contains_key(&class.obj_id) {
            return Err(Error::new(io::ErrorKind::Other,"Duplicate bootstrap oid"));
        }
        
        classes_map.insert(class.obj_id, class.clone());

        Ok(())
    }
}
