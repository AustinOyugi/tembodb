use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::{io, process};
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn get_configs_from_file() -> Lines<BufReader<File>> {
    match read_lines("tembodata/tembodb.conf") {
        Ok(lines) => {
            lines
        }
        Err(error) => {
            eprintln!("Error loading tembo configuration file!! {}", error);
            process::exit(1)
        }
    }
}

pub (crate)  fn extract_value_mapper() -> HashMap<String,String> {
    let  file_contents = get_configs_from_file();
    let mut config_values: HashMap<String,String> = HashMap::new();
    for  line_value in file_contents.map_while(Result::ok)  {
        let mut splits = line_value.split("=");
        match splits.next() {
            None => {}
            Some(key) => {
                match splits.next() {
                    None => {}
                    Some(value) => {
                        config_values.insert(
                            key.parse().unwrap(),
                            value.parse().unwrap());
                    }
                }
            }
        }
    }
    config_values
}
