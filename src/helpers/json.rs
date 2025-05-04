use serde_json::{Value, json};
use std::fs::{self};
use std::path::Path as FilePath;

use crate::consts::FOLDER_NAME;

use super::file::create_file;

pub fn read_json(file_name: &str) -> Value {
    let path = format!("{}/{}.json", FOLDER_NAME, file_name);
    if !FilePath::new(&path).exists() {
        create_file(file_name);
        return json!([]); // Default to empty array
    }
    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| json!([]))
}

pub fn write_to_json(file_name: &str, data: &Value) {
    let json = serde_json::to_string_pretty(data).unwrap();
    fs::write(format!("{}/{}.json", FOLDER_NAME, file_name), json).unwrap()
}
