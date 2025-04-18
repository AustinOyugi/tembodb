use std::collections::HashMap;

/// A segment represents a collection of grouped pages
/// Each segment contains the first page, which contains metadata about the segment
/// The first page that contains metadata about the segment
///  64000 bits  8000 bytes 8kb
/// (4 + 4 + 2) = 10 bytes for fields
/// In a 64 bits arch i.e. 64/8 = 8 bytes per cpu cycle
/// We pad the first 2 fields 4 + 4 : so that accessing them fills the buffer
/// The last field which is 2 bytes we pad 6 bytes
/// Lastly we need to fill in the page till it gets to 8kb
/// 8192 - 16 = 8176
#[repr(C)]
pub struct TemboPageZero {
    total_pages: u32,
    next_free_page: u32,
    page_size: u16,
    _pad : [u8; 6],
    reserved: [u8; 8176]
}

impl TemboPageZero {
    pub fn new() -> Self {
        TemboPageZero {
            total_pages: 1,
            next_free_page: 1,
            page_size: 8192,
            _pad: [0; 6],
            reserved: [0; 8176],
        }
    }
}

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
    length: u32,
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
            records: Vec::with_capacity(0),
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
