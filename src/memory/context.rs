use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A memory context is an organized memory structure that
/// 1. Groups a bunch of allocations together logically
/// 2. Tracks all memory allocated to it
/// 3. Provides a way to free all allocations at once
/// 4. Maintains a relationship between different allocation zones
/// To get to the point:
/// instead of using the OS memory allocation functions, we abstract
/// the functionalities and provide a better memory management feature
/// where we decide according to relation where we want to store the objects
/// in memory
/// This allows us to efficiently manage the block of allocation.
pub struct TemboMemoryContext{
    /// The name of the memory context
    name: String,

    /// The owner of the memory context
    parent: Option<Arc<Mutex<TemboMemoryContext>>>,

    /// The children of the memory context
    /// We are okay having it as an array for now
    children: Vec<Option<Arc<Mutex<TemboMemoryContext>>>>,

    /// All allocations required and their metadata
    allocations: HashMap<usize, Allocation>
}

pub struct Allocation {

    // The actual memory allocated
    size: usize

    // Other metadata
}

impl TemboMemoryContext{
    
    pub fn new(name: &str, parent: Option<Arc<Mutex<TemboMemoryContext>>> ) -> Arc<Mutex<Self>> {
        let tembo_context = Arc::new(Mutex::new(Self {
            name: name.to_string(),
            parent: parent.clone(),
            children: Vec::new(),
            allocations: Default::default(),
        }));

        // Checks if the parent exists and if he does then we access the arc
        if let Some(tp) = parent {

            // Blocks and waits until the parent can be accessed
            if let Ok (mut tp_locked) = tp.lock(){

                // Get the children lists and adds the current context as a child
                tp_locked.children.push(Option::from(tembo_context.clone()))
            }
        };

        tembo_context
    }

    pub fn alloc(&mut self, size: usize) -> usize {
        // Protects the counter value and is accessible throughout the program lifecycle
        static COUNTER: AtomicUsize = AtomicUsize::new(1);

        // Get the latest pointer and increments it, we use the highest ordering
        // to make sure every thread sees this
        let alloc_ptr_id = COUNTER.fetch_add(1, Ordering::SeqCst);

        // Allocate memory
        self.allocations.insert(alloc_ptr_id, Allocation { size });

        // Return the allocation id
        alloc_ptr_id
    }
}
