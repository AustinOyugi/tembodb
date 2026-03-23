const PAGE_SIZE: usize = 8192;

#[repr(C)]
pub struct TemboPageZero {

    // supposed to track the page serial version, e.g b"Tembo\0\0\0"
    // more like the type of file
    // passport
    magic: [u8; 8],

    // the file format version
    // language, what processor would read the page
    version: u16,

    // The total size a page should be
    page_size: u16,

    // Total number of pages inside the segment
    total_pages: u32,

    // the id of the first page
    root_page: u32,

    // Total size
    // 8 + 2 + 2 + 4 + 4 =  20 bytes
    // 8192 - 20  = 8172 bytes left
    reserved: [u8; 8172],
}

#[repr(C)]
#[derive(Debug)]
pub struct TemboPageHeader {
    // Unique identifier for the page
    id: u32,

    // Stores the number of slots available in the page
    slot_count: u16,

    // Tells where the next slot should be allocated
    free_start: u16,

    // Tells where the record should be written at, from the end
    free_end: u16,

    // we are aligning in terms of 4 bytes u32
    _pad: u16
}

/// Total size
/// 4 + 1 = 5
/// padding required is 3
///  Total is 8 bytes
#[repr(C)]
#[derive(Debug)]
pub struct LinePointer {
    // Distance from the start where the tuple is stored
    offset: u8,
    // The size of the tuple
    length: u32,
    _pad: [u8; 3],
}

#[repr(C)]
#[derive(Debug)]
pub struct TemboPage {
    data: [u8; PAGE_SIZE],
}


pub struct BufferPool {
    cache: Vec<[u8; 8192]>,
    dirty_pages: Vec<u32>,
}
