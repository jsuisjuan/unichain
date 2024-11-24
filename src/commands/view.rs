use serde_json;

use crate::get_file;
use crate::model::FileError;
use crate::utils::prompt_for_file_id;

const PATH: &str = "../assets";

pub fn view_file() -> Result<(), FileError> {
    let file_id = prompt_for_file_id()?;
    let file = get_file(PATH, file_id)?;
    println!("\nFiles:\n{}", serde_json::to_string_pretty(&file).unwrap());
    Ok(())
}