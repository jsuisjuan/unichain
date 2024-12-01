use log::{info, warn};

use crate::remove_file;
use crate::model::FileError;
use crate::utils::prompt_for_file_id;

pub fn delete_file() -> Result<(), FileError> {
    loop {
        let file_id = prompt_for_file_id()?;
        if let Err(_) = remove_file(file_id) {
            warn!("File not found. Please check if ID is correct.");
            continue;
        }
        info!("File ID {:?} was moved to the trash.", file_id);
        return Ok(());
    }
}