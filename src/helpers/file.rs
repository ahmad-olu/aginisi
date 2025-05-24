use std::fs::{self, File};
use std::path::Path as FilePath;

use crate::consts::FOLDER_NAME;
use crate::model::error::Error;

pub fn create_file(file_name: &str) -> Result<(), Error> {
    if !FilePath::new(&format!("{}/{}.json", FOLDER_NAME, file_name)).exists() {
        let _ = File::create(format!("{}/{}.json", FOLDER_NAME, file_name))
            .map_err(|error| Error::FileError(error));
    }
    Ok(())
}

pub fn open_file(file_name: &str) -> Result<File, Error> {
    File::open(format!("{}/{}.json", FOLDER_NAME, file_name))
        .map_err(|error| Error::FileError(error))
}

pub fn delete_file(file_name: &str) -> Result<(), Error> {
    fs::remove_file(format!("{}/{}.json", FOLDER_NAME, file_name))
        .map_err(|error| Error::FileError(error))
}
