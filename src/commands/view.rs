use log::warn;
use serde_json;

use crate::get_file;
use crate::model::FileError;
use crate::utils::prompt_for_file_id;

pub fn view_file() -> Result<(), FileError> {
    loop {
        let file_id = prompt_for_file_id()?;
        if let Ok(file) = get_file(file_id) {
            println!("\nFiles:\n{}", serde_json::to_string_pretty(&file).unwrap());
            return Ok(());
        }
        warn!("File not found.");
        continue;
    }
}