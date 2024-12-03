use log::info;
use serde_json;

use crate::model::{File,FileError};
use crate::load_files_from_file;

pub fn list_files() -> Result<(), FileError> {
    info!("Fetching all the files.");
    let files: Vec<File> = match load_files_from_file() {
        Ok(files) => files,
        Err(_) => Err(FileError::FileNotFound)?
    };
    info!("Successfully fetched {} files.", files.len());
    println!("\nFiles:\n{}", serde_json::to_string_pretty(&files).unwrap());
    Ok(())
}