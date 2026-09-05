use std::fs::{File, OpenOptions};
use std::io;

pub fn create_redirect_file(path: &str, append: bool) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(path)
}