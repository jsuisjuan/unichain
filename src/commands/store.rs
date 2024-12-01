use std::path::PathBuf;

use log::warn;

use crate::create_new_file;
use crate::model::{FileData, FileError};
use crate::utils::{get_system_owner, process_input, handle_input};

pub fn store_file() -> Result<(), FileError> {
    let file_path = setup_input("\nInsert file path you want to store: ", None)?;
    let filename = extract_filename(&file_path)?;
    let final_name = setup_input(&format!("\nYour current file name is: {}. Do you want to change it? (Y/N): ", filename.display()), Some(&filename))?;
    let file_data = FileData { owner: get_system_owner(), name: final_name };
    create_new_file(file_data, &file_path)?;
    Ok(())
}

fn setup_input<T: From<String>>(prompt: &str, file_name: Option<&PathBuf>) -> Result<T, FileError> {
    loop {
        print!("{}", prompt);
        let response = handle_input()?;
        let result = match file_name {
            Some(name) => change_filename(&response, name).map(|path| path.to_string()),
            None => get_file_path(&response).map(|path| path.to_str().unwrap().to_string())
        };
        match result {
            Ok(value) => return Ok(value.into()),
            Err(_) => continue,
        }
    }
}

fn extract_filename(path: &PathBuf) -> Result<PathBuf, FileError> {
    match path.file_name() {
        Some(name) => Ok(name.to_string_lossy().into_owned().into()),
        None => Err(FileError::InputError("Invalid file path".to_string()))
    }
}

fn change_filename(choosed_option: &String, current_name: &PathBuf) -> Result<String, FileError> {
    let option = choosed_option.trim().to_lowercase();
    if option == "y" {
        match process_input("Add new file name: ", false)? {
            Some(new_name) => Ok(new_name),
            None => Err(FileError::InputError("New file name cannot be empty.".to_string())),
        }
    } else {
        Ok(current_name.to_string_lossy().into_owned())
    }
}

fn get_file_path(file_path: &String) -> Result<PathBuf, FileError> {
    let path = file_path.trim();
    if path.is_empty() {
        warn!("File path cannot be empty. Please try again.");
        return Err(FileError::InputError("Empty path".to_string()));
    }
    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        warn!("File not found at path: {:?}. Please try again.", path_buf);
        return Err(FileError::FileNotFound);
    }
    Ok(path_buf)
}