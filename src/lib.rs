use std::fs::File;
use std::io::{BufReader};


pub fn file_reader(file_loc: &str) -> BufReader<File> {
    let file = File::open(file_loc).unwrap();
    let reader = BufReader::new(file);
    reader
}

