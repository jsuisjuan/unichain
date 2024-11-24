use log::{info, error};

use crate::model::{File,FileError};
use crate::get_all_files;

pub const PATH: &str = "../assets";

pub fn list_files() -> Result<(), FileError> {
    info!("Fetching all the files.");
    let files: Vec<File> = match get_all_files(PATH) {
        Ok(files) => files,
        Err(e) => {
            error!("Failed to fetch files: {}", e);
            return Err(FileError::FileNotFound);
        }
    };
    info!("Successfully fetched {} files.", files.len());
    println!("\n{:?}", files);
    Ok(())
}