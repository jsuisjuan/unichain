use std::fs::File as StdFile;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::env;

use bincode;
use dotenv::dotenv;

pub mod model;
pub mod commands;
pub mod utils;

use model::{File, FileData, FileError};
use utils::{get_default_file, process_modified_file, update_accessed_file_date};

const DEFAULT_PATH: &str = "../assets";

fn get_path() -> String {
    dotenv().ok();
    env::var("ASSETS_PATH").unwrap_or_else(|_| DEFAULT_PATH.to_string())
}

fn load_files_from_file() -> Result<Vec<File>, FileError> {
    let mut file = match StdFile::open(&get_path()) {
        Ok(f) => f,
        Err(_) => return Ok(Vec::new()),
    };
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| FileError::IOError(e))?;
    let decoded = bincode::deserialize(&buffer).map_err(|_| FileError::DeserializationError("Failed to deserialize Vec<File>".to_string()))?;
    Ok(decoded)
}

fn save_files_to_file(files: &Vec<File>) -> Result<(), FileError> {
    let encoded = bincode::serialize(files).map_err(|_| FileError::DeserializationError("Vec<File> serialization failed".to_string()))?;
    let mut file = StdFile::create(&get_path()).map_err(|e| FileError::IOError(e))?;
    file.write_all(&encoded).map_err(|e| FileError::IOError(e))?;
    Ok(())
}

fn add_files_to_file(file: File) -> Result<(), FileError> {
    let mut files = load_files_from_file()?;
    files.push(file);
    save_files_to_file(&files)
}

pub fn create_new_file(file_data: FileData, file_path: &PathBuf) -> Result<(), FileError> {
    let mut file = get_default_file(&file_data, file_path).map_err(|e| FileError::InputError(format!("Error creating file: {}", e)))?;
    file.name = file_data.name;
    add_files_to_file(file).map_err(|e| FileError::IOError(match e {
        FileError::IOError(err) => err,
        _ => io::Error::new(io::ErrorKind::Other, "Unknown error"),
    }))?;
    Ok(())
}

pub fn get_all_files() -> Result<Vec<File>, FileError> {
    load_files_from_file().map_err(|e| FileError::IOError(match e {
        FileError::IOError(err) => err,
        _ => io::Error::new(io::ErrorKind::Other, "Error loading file"),
    }))
}

pub fn get_file(file_id: i64) -> Result<File, FileError> {
    let mut files = get_all_files()?;
    let file_index = files.iter().position(|file| file.id == file_id).ok_or(FileError::FileNotFound)?;
    {
        let file = &mut files[file_index];
        *file = update_accessed_file_date(file.clone())?;
    }
    save_files_to_file(&files)?;
    Ok(files[file_index].clone())
}

pub fn modify_file(file_id: i64, updated_file: File) -> Result<(), FileError> {
    let mut files = get_all_files()?;
    let file_index = files.iter().position(|file| file.id == file_id).ok_or(FileError::FileNotFound)?;
    files[file_index] = process_modified_file(updated_file)?;
    save_files_to_file(&files)?;
    Ok(())
}

pub fn remove_file(file_id: i64) -> Result<(), FileError> {
    let mut files = get_all_files()?;
    let initial_length = files.len();
    files.retain(|file| file.id != file_id);
    if files.len() == initial_length {
        return Err(FileError::FileNotFound);
    }
    save_files_to_file(&files)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use chrono::Utc;
    use tempfile::tempdir;
    use utils::generate_fake_hash;

    fn get_test_file() -> File {
        let owner = (1, String::from("Username"), String::from("username@gmail.com"));
        let owner_access = owner.clone();
        File { id: 1, name: "test-file".to_string(), file_type: model::FileType::Pdf, size: 100, created: Utc::now().naive_utc(), modified: None, accessed: None, owner, people_with_access: vec![owner_access], ipfs_hash: generate_fake_hash(46), onchain_txn_id: generate_fake_hash(64), download_permission: false, description: None }
    }

    #[test]
    fn test_load_files_from_empty_file() {
        let tem_dir = tempdir().unwrap();
        let test_file_path = tem_dir.path().join("test_file.bin");
        env::set_var("ASSETS_PATH", test_file_path.to_str().unwrap());
        let result = load_files_from_file();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::new());
        fs::remove_file(test_file_path).ok();
    }

    #[test]
    fn test_save_and_load_files() {
        let tem_dir = tempdir().unwrap();
        let test_file_path = tem_dir.path().join("test_file.bin");
        env::set_var("ASSETS_PATH", test_file_path.to_str().unwrap());
        let file = get_test_file();
        let files = vec![file.clone()];
        let save_result = save_files_to_file(&files);
        assert!(save_result.is_ok());
        let load_result = load_files_from_file();
        assert!(load_result.is_ok());
        assert_eq!(load_result.unwrap(), files);
        fs::remove_file(test_file_path).ok();
    }
}