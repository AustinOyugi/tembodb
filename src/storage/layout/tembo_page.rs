use std::collections::HashMap;

// 16 kilo bytes
static TEMBO_PAGE_SIZE: usize = 8192 * 16;
// 24 bytes
static TEMBO_HEADER_SIZE: u8 = 24;
// 4 bytes
static TEMBO_POINTER_SIZE: u8 = 4;

#[derive(Debug)]
pub struct TemboPageHeader {
    id: u32,
    // Stores the number of records stored in the page
    record_count: u8,
}

#[derive(Debug)]
pub struct LinePointer {

    // Distance from the start where the tuple is stored
    offset: u8,

    // The size of the tuple
    length: u32
}

#[derive(Debug)]
pub struct TemboPage {
    // Store metadata about the page
    page_header: TemboPageHeader,

    // Line Pointer Array
    line_pointers: Vec<LinePointer>,

    // The actual store of data for the page
    records: Vec<u8>,
}

impl TemboPage {
    pub fn new() -> Self {
        Self {
            page_header: TemboPageHeader {
                id: 0,
                record_count: 0,
            },
            line_pointers: vec![],
            records: Vec::with_capacity(TEMBO_PAGE_SIZE),
        }
    }

    // pub  fn get_page_header(&self)  {
    // }
}

pub struct BufferPool {
    cache: HashMap<u32, TemboPage>,
    dirty_pages: HashMap<u32, TemboPage>,
}

struct FreeSpaceMap {
    free_pages: Vec<u32>,
}
