use std::fs::File as StdFile;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use bincode;

pub mod model;
use model::{File, FileData, FileError};

pub mod utils;
use utils::{get_default_file, process_modified_file, update_accessed_file_date};

fn load_files_from_file(path: &str) -> Result<Vec<File>, FileError> {
    let mut file = match StdFile::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(Vec::new()),
    };
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| FileError::IOError(e))?;
    let decoded = bincode::deserialize(&buffer).map_err(|_| FileError::DeserializationError("Failed to deserialize Vec<File>".to_string()))?;
    Ok(decoded)
}

fn save_files_to_file(files: &Vec<File>, path: &str) -> Result<(), FileError> {
    let encoded = bincode::serialize(files).map_err(|_| FileError::DeserializationError("Vec<File> serialization failed".to_string()))?;
    let mut file = StdFile::create(path).map_err(|e| FileError::IOError(e))?;
    file.write_all(&encoded).map_err(|e| FileError::IOError(e))?;
    Ok(())
}

fn add_files_to_file(file: File, path: &str) -> Result<(), FileError> {
    let mut files = load_files_from_file(path)?;
    files.push(file);
    save_files_to_file(&files, path)
}

pub fn create_new_file(file_data: FileData, file_path: &PathBuf, path: &str) -> Result<(), FileError> {
    let mut file = get_default_file(&file_data, file_path).map_err(|e| FileError::InputError(format!("Error creating file: {}", e)))?;
    file.name = file_data.name;
    add_files_to_file(file, path).map_err(|e| FileError::IOError(match e {
        FileError::IOError(err) => err,
        _ => io::Error::new(io::ErrorKind::Other, "Unknown error"),
    }))?;
    println!("File created and saved successfully");
    Ok(())
}

pub fn get_all_files(path: &str) -> Result<Vec<File>, FileError> {
    load_files_from_file(path).map_err(|e| FileError::IOError(match e {
        FileError::IOError(err) => err,
        _ => io::Error::new(io::ErrorKind::Other, "Error loading file"),
    }))
}

pub fn get_file(path: &str, file_id: i64) -> Result<File, FileError> {
    let mut files = get_all_files(path)?;
    let file_index = files.iter().position(|file| file.id == file_id).ok_or(FileError::FileNotFound)?;
    {
        let file = &mut files[file_index];
        *file = update_accessed_file_date(file.clone())?;
    }
    save_files_to_file(&files, path)?;
    Ok(files[file_index].clone())
}

pub fn modify_file(path: &str, file_id: i64, updated_file: File) -> Result<(), FileError> {
    let mut files = get_all_files(path)?;
    let file_index = files.iter().position(|file| file.id == file_id).ok_or(FileError::FileNotFound)?;
    files[file_index] = process_modified_file(updated_file)?;
    save_files_to_file(&files, path)?;
    Ok(())
}

pub fn delete_file(path: &str, file_id: i64) -> Result<(), FileError> {
    let mut files = get_all_files(path)?;
    let initial_length = files.len();
    files.retain(|file| file.id != file_id);
    if files.len() == initial_length {
        return Err(FileError::FileNotFound);
    }
    save_files_to_file(&files, path)?;
    Ok(())
}