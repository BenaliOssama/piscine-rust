use std::fs::File;
use std::io::prelude::*;

pub fn open_file(s: &str) -> File {
    let mut file = File::open(s).unwrap();
    file
}
