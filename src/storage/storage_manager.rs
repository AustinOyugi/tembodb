use std::fs;

pub fn initialize_storage() -> std::io::Result<()> {
    /*
           Load the data storage
    */

    // Check if the storage is ready
    fs::create_dir_all("tembodata/storage")?;

    // let mut  tembo_page:  TemboPage = TemboPage::new();
    //
    // println!("{:?}",tembo_page);

    fs::write("tembodata/storage/text.txt", b"Hello world")?;
    //
    // let contents = fs::read("tembodata/storage/text.txt")?;

    Ok(())
}
