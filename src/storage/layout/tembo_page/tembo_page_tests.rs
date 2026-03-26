use super::*;
use std::fs::OpenOptions;

#[test]
fn test_movements() {
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .read(true)
        .open("tembo.db");

    assert!(file.is_ok());
    println!("File created!");

    let mut file = file.unwrap();

    let page_zero_bytes = get_page_zero_bytes();
    let write_page_zero = write_to_file(&mut file, &page_zero_bytes, 0);
    assert!(write_page_zero.is_ok());
    println!("Write page zero!");

    let get_tembo_header_bytes = get_tembo_header_bytes();
    println!("Size of tembo header: {}", get_tembo_header_bytes.len());
    let write_tembo_header =
        write_to_file(&mut file, &get_tembo_header_bytes, (PAGE_SIZE as u64) - 1);
    assert!(write_tembo_header.is_ok());
    println!("Write page header!");

    let page_zero = read_page_zero_from_file(&mut file);
    assert!(page_zero.is_ok());
    let page_zero = page_zero.unwrap();
    assert_eq!(page_zero.page_size, PAGE_SIZE);
    assert_eq!(page_zero.total_pages, 1);
    assert_eq!(page_zero.root_page, 1);
    assert_eq!(page_zero.magic, TEMBO_MAGIC);
    assert_eq!(page_zero.version, 0);
    println!("Read page zero!");

    let tembo_page_header = read_tembo_header(&mut file);
    assert!(tembo_page_header.is_ok());
    let tembo_page_header = tembo_page_header.unwrap();
    assert_eq!(tembo_page_header.id, 0);
    assert_eq!(tembo_page_header.slot_count, 0);
    assert_eq!(
        tembo_page_header.free_start,
        size_of::<TemboPageHeader>() as u16
    );
    assert_eq!(tembo_page_header.free_end, PAGE_SIZE);
}
