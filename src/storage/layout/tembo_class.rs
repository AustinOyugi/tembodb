use crate::memory::context::TemboMemoryContext;
use std::sync::{Arc, Mutex};

/// Defines the type of relations
/// A table, index or a view
pub enum RelType {
    Table,
    Index,
    View,
}

/// A tembo relation definition represents a relation
/// Stores, metadata about what was persisted
pub struct TemboRelDef {
    /// The object identifier
    pub obj_id: u32,

    /// Relationship name
    pub rel_name: String,

    /// Type of relationship
    /// Either table(t) or index (i)
    pub rel_type: RelType,

    /// The number of pages that exists
    pub rel_pages: u32,

    /// The memory context tied to the relation
    pub memory_context: Arc<Mutex<TemboMemoryContext>>,
}

impl TemboRelDef {
    pub fn bootstrap(context: Arc<Mutex<TemboMemoryContext>>) -> Self {
        Self {
            obj_id: 1,
            rel_name: "tembo_class".to_string(),
            rel_type: RelType::Table,
            rel_pages: 0,
            memory_context: context.clone(),
        }
    }
}
