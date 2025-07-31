use std::path::Path;
use std::fs::File;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = File::open(path).unwrap_or_else(|_|{
        File::create(path).unwrap();
        File::open(path).unwrap()
    });
}
