use log::info;

use crate::remove_file;
use crate::model::FileError;
use crate::utils::prompt_for_file_id;

const PATH: &str = "../assets";

pub fn delete_file() -> Result<(), FileError> {
    let file_id = prompt_for_file_id()?;
    remove_file(PATH, file_id)?;
    info!("\nFile ID {} was moved to the trash.", file_id);
    Ok(())
}