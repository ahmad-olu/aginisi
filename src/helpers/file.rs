use std::fs::{self, File};
use std::path::Path as FilePath;

use crate::consts::FOLDER_NAME;

pub fn create_file(file_name: &str) {
    if !FilePath::new(&format!("{}/{}.json", FOLDER_NAME, file_name)).exists() {
        File::create(format!("{}/{}.json", FOLDER_NAME, file_name)).unwrap();
    }
}

pub fn open_file(file_name: &str) -> File {
    File::open(format!("{}/{}.json", FOLDER_NAME, file_name)).unwrap()
}

pub fn delete_file(file_name: &str) -> () {
    fs::remove_file(format!("{}/{}.json", FOLDER_NAME, file_name)).unwrap()
}
