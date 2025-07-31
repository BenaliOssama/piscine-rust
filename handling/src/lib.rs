use std::fs::OpenOptions;
use std::path::Path;
use std::io::{self, Write, ErrorKind};
use std::fs::File;


pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path).unwrap();

    file.write_all(content.as_bytes()).unwrap();
}



