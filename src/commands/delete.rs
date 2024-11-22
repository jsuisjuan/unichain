use std::io::{self, Write};
use log::{info, error};
use crate::model::FileError;
use crate::get_file;

pub fn delete_file() -> Result<(), FileError> {
    print!("Insert file ID: ");
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut file_id_input = String::new();
    io::stdin().read_line(&mut file_id_input).map_err(|e| FileError::IOError(e))?;
    let file_id: i64 = file_id_input.trim().parse::<i64>().map_err(|_| FileError::ParseError)?;
    remove_file(PATH, file_id)?;
    println!("File ID {} was moved to the trash!", file_id);
    Ok(())
}