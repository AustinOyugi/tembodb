use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use bytemuck::{Pod, Zeroable};

const PAGE_SIZE: u16 = 8192;

#[repr(C)]
#[derive(Debug)]
pub struct TemboPage {
    data: [u8; PAGE_SIZE as usize],
}


// Total 4 + 2 + 2 + 2 + 2 =  12 bytes
#[repr(C)]
#[derive(Debug,Copy, Clone, Pod, Zeroable)]
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

impl TemboPageHeader {
    pub fn root(id: u32) -> Self {
        TemboPageHeader {
            id,
            slot_count: 0,
            free_start: size_of::<TemboPageHeader>() as u16,
            free_end: PAGE_SIZE,
            _pad: 0,
        }
    }
}



#[repr(C)]
#[derive(Debug)]
pub struct LinePointer {
    // Where the record starts inside the page
    offset: u16,
    // The size of the tuple
    length: u16,
}


/// The first page that gives metadata about the pages
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
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

const TEMBO_MAGIC: [u8;8] = *b"TEMBO\0\0\0";

impl TemboPageZero {
    pub fn new() -> TemboPageZero {
        TemboPageZero {
            magic: TEMBO_MAGIC,
            version: 0,
            page_size: PAGE_SIZE,
            total_pages: 1,
            root_page: 1,
            reserved: [0u8;8172],
        }
    }
}


pub struct BufferPool {
    cache: Vec<[u8; 8192]>,
    dirty_pages: Vec<u32>,
}


fn get_page_zero_bytes() -> [u8; PAGE_SIZE as usize] {
    let tembo_page_zero = &TemboPageZero::new();
    const _: () = assert!(size_of::<TemboPageZero>() == 8192);
    let page_zero_bytes = bytemuck::bytes_of(tembo_page_zero);
    let mut zero_raw_page_buffer = [0u8;PAGE_SIZE as usize];
    zero_raw_page_buffer[..size_of::<TemboPageZero>()].copy_from_slice(page_zero_bytes);
    zero_raw_page_buffer
}


fn get_tembo_header_bytes() -> [u8; PAGE_SIZE as usize] {
    let tembo_page_header = &TemboPageHeader::root(0);
    let header_page_bytes = bytemuck::bytes_of(tembo_page_header);
    let mut raw_page_buffer = [0u8; PAGE_SIZE as usize];
    raw_page_buffer[..size_of::<TemboPageHeader>()].copy_from_slice(header_page_bytes);
    raw_page_buffer
}


fn read_page_zero_from_file(file: &mut File) -> std::io::Result<TemboPageZero> {
    let read_file = read_from_file(file, 0)?;
    let read_page_zero: &TemboPageZero = bytemuck::from_bytes(&read_file);
    Ok(read_page_zero.clone())
}

fn read_tembo_header(file: &mut File) -> std::io::Result<TemboPageHeader> {
    let read_file = read_from_file(file, (PAGE_SIZE as u64) -1)?;
    let header_section = read_file[..size_of::<TemboPageHeader>()].to_vec();
    let read_page_header: &TemboPageHeader = bytemuck::from_bytes(&header_section);
    Ok(read_page_header.clone())
}

fn read_from_file(file: &mut File, offset: u64) -> std::io::Result<[u8; PAGE_SIZE as usize]> {
    let mut buffer = [0u8; PAGE_SIZE as usize];
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}

fn write_to_file(file: &mut File, data: &[u8; PAGE_SIZE as usize], offset: u64) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(data)?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tembo_page_tests;