use std::io::{self, Write};
use log::{info, error};

use crate::get_file;
use crate::model::FileError;

const PATH: &str = "../assets/";

pub fn view_file() -> Result<(), FileError> {
    print!("Insert file ID: ");
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    
    let mut file_id_input = String::new();
    io::stdin().read_line(&mut file_id_input).map_err(|e| FileError::IOError(e))?;
    
    let file_id = file_id_input.trim().parse::<i64>().map_err(|_| FileError::ParseError)?;
    
    match get_file(PATH, file_id) {
        Ok(file) => {
            info!("File found\n{:?}", file);
            Ok(())
        },
        Err(_) => {
            error!("File not found.");
            Err(FileError::FileNotFound)
        }
    }
}