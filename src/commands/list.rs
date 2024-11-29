use log::info;
use serde_json;

use crate::model::{File,FileError};
use crate::{load_files_from_file, get_path};

pub fn list_files() -> Result<(), FileError> {
    print!("\n");
    info!("Fetching all the files.");
    let files: Vec<File> = match load_files_from_file(&get_path()) {
        Ok(files) => files,
        Err(_) => Err(FileError::FileNotFound)?
    };
    info!("Successfully fetched {} files.", files.len());
    println!("\nFiles:\n{}", serde_json::to_string_pretty(&files).unwrap());
    Ok(())
}