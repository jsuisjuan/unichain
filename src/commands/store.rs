use std::io::{self, Write};
use log::{info, error};
use crate::model::{FileData, FileError};
use crate::create_new_file;

pub fn store_file() -> Result<(), FileError> {
    let file_path = prompt_for_file_path()?;
    let filename = extract_filename(&file_path)?;
    let final_name = ask_for_filename_change(&filename)?;
    let file_data = FileData {
        owner: get_system_owner(),
        name: final_name,
    };
    create_new_file(file_data, &file_path, PATH)?;
    Ok(())
}

fn prompt_for_file_path() -> Result<PathBuf, FileError> {
    print!("Insert file path you want to store: ");
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).map_err(|e| FileError::IOError(e))?;
    let trimmed_path = file_path.trim();
    if trimmed_path.is_empty() {
        error!("File path input is empty.");
        return Err(FileError::InputError("File path cannot be empty.".to_string()));
    }
    Ok(PathBuf::from(trimmed_path))
}

fn extract_filename(path: &PathBuf) -> Result<PathBuf, FileError> {
    match path.file_name() {
        Some(name) => Ok(name.to_string_lossy().into_owned().into()),
        None => {
            error!("Invalid file path: {}", path.display());
            Err(FileError::InputError("Invalid file path".to_string()))
        }
    }
}

fn ask_for_filename_change(current_name: &PathBuf) -> Result<String, FileError> {
    print!("Your current file name is: {}. Do you want to change it? (Y/N): ", current_name.display());
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut response = String::new();
    io::stdin().read_line(&mut response).map_err(|e| FileError::IOError(e))?;
    let trimmed_response = response.trim().to_lowercase();
    if trimmed_response == "y" {
        match process_input("Add new file name: ", false)? {
            Some(new_name) => Ok(new_name),
            None => Err(FileError::InputError("File name cannot be empty.".to_string())),
        }
    } else {
        Ok(current_name.to_string_lossy().into_owned())
    }
}