use std::fs::File as StdFile;
use std::io::{Read, Write};
use std::path::PathBuf;

use bincode;

pub mod model;
pub mod commands;
pub mod utils;

use model::{File, FileData, FileError};
use utils::{get_default_file, process_modified_file, update_accessed_file_date};

const FILE_NAME: &str = "files.bin";

pub fn load_files_from_file() -> Result<Vec<File>, FileError> {
    let mut file = StdFile::open(FILE_NAME).map_err(|e| FileError::IOError(e))?;
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded).map_err(|e| FileError::IOError(e))?;
    if encoded.is_empty() {
        return Ok(Vec::new());
    }
    bincode::deserialize(&encoded).map_err(|_| FileError::DeserializationError("Failed to deserialize Vec<File>".to_string()))
}

fn save_files_to_file(files: &Vec<File>) -> Result<(), FileError> {
    let encoded = bincode::serialize(files).map_err(|_| FileError::DeserializationError("Vec<File> serialization failed".to_string()))?;
    let mut file = StdFile::create(FILE_NAME).map_err(|e| FileError::IOError(e))?;
    file.write_all(&encoded).map_err(|e| FileError::IOError(e))?;
    Ok(())
}

pub fn create_new_file(file_data: FileData, file_path: &PathBuf) -> Result<(), FileError> {
    let mut files = load_files_from_file()?;
    let mut file = get_default_file(&file_data, file_path).map_err(|e| FileError::InputError(format!("Error creating file: {}", e)))?;
    file.name = file_data.name;
    files.push(file);
    save_files_to_file(&files)?;
    Ok(())
}

pub fn get_file(file_id: i64) -> Result<File, FileError> {
    let mut files = load_files_from_file()?;
    let file_index = files.iter().position(|file| file.id == file_id).ok_or(FileError::FileNotFound)?;
    {
        let file = &mut files[file_index];
        *file = update_accessed_file_date(file.clone())?;
    }
    save_files_to_file(&files)?;
    Ok(files[file_index].clone())
}

pub fn modify_file(file_id: i64, updated_file: File) -> Result<(), FileError> {
    let mut files = load_files_from_file()?;
    let file_index = files.iter().position(|file| file.id == file_id).ok_or(FileError::FileNotFound)?;
    files[file_index] = process_modified_file(updated_file)?;
    save_files_to_file(&files)?;
    Ok(())
}

pub fn remove_file(file_id: i64) -> Result<(), FileError> {
    let mut files = load_files_from_file()?;
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
    use std::{env, fs::{self, File as FsFile},path::PathBuf};
    use std::io::Write;

    use chrono::Utc;

    use utils::generate_fake_hash;

    fn get_test_file() -> File {
        let owner = (1, String::from("Username"), String::from("username@gmail.com"));
        let owner_access = owner.clone();
        File { 
            id: 1, name: "test-file".to_string(), file_type: model::FileType::Pdf, size: 100, 
            created: Utc::now().naive_utc(), modified: None, accessed: None, owner, 
            people_with_access: vec![owner_access], ipfs_hash: generate_fake_hash(46), 
            onchain_txn_id: generate_fake_hash(64), download_permission: false, description: None 
        }
    }

    fn setup_temp_file() -> (PathBuf, FsFile) {
        let temp_dir = PathBuf::from("./test_temp_dir");
        if !temp_dir.exists() {
            fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");
        }
        let test_file_path = temp_dir.join("test_file.bin");
        let test_file = FsFile::create(&test_file_path).expect("Failed to create test file");
        env::set_var("ASSETS_PATH", test_file_path.to_str().unwrap());
        (test_file_path, test_file)
    }

    fn create_fake_pdf_in_other_dir(directory: &PathBuf) -> PathBuf {
        let fake_pdf_path = directory.join("fake_test_file.pdf");
        let fake_pdf_content = b"%PDF-1.4\n%...\n%%EOF"; 
        let mut file = FsFile::create(&fake_pdf_path).expect("Failed to create fake PDF file");
        file.write_all(fake_pdf_content).expect("Failed to write to fake PDF file");
        fake_pdf_path
    }

    #[test]
    fn test_load_files_from_empty_file() {
        let (test_file_path, _temp_dir) = setup_temp_file();
        StdFile::create(&test_file_path).expect("Failed to create an empty test file");
        let result = load_files_from_file();
        assert!(result.is_ok(), "Load failed: {:?}", result);
        assert_eq!(result.unwrap(), Vec::<File>::new(), "Expected an empty file");
        env::remove_var("ASSETS_PATH");
    } 

    fn save_file() -> (PathBuf, FsFile, Vec<File>) {
        let (test_file_path, temp_dir) = setup_temp_file();
        let file = get_test_file();
        let files = vec![file.clone()];
        let save_result = save_files_to_file(&files);
        assert!(save_result.is_ok(), "Save failed: {:?}", save_result);
        assert!(test_file_path.exists(), "File was not created at: {:?}", test_file_path);
        (test_file_path, temp_dir, files)
    }

    #[test]
    fn test_save_and_load_files() {
        let (test_file_path, _temp_dir, files) = save_file();
        let load_result = load_files_from_file();
        assert!(load_result.is_ok(), "Load failed: {:?}", load_result);
        assert_eq!(load_result.unwrap(), files, "Loaded files do not match saved files");
        env::remove_var("ASSETS_PATH");
    }

    #[test]
    fn test_create_new_file() {
        let (test_file_path, _file) = setup_temp_file();
        let temp_dir = PathBuf::from("./test_temp_dir");
        let fake_pdf_path = create_fake_pdf_in_other_dir(&temp_dir);
        assert!(fake_pdf_path.exists(), "Fake PDF file not created");
        let file_data = FileData { owner: (1, String::from("username"), String::from("username@gmail.com")), name: "test-file.pdf".to_string() };
        let create_result = create_new_file(file_data.clone(), &fake_pdf_path);
        assert!(create_result.is_ok(), "Failed to create new file: {:?}", create_result);
        let files = load_files_from_file().expect("Failed to load files");
        assert_eq!(files.len(), 1, "Expected one file in the list");
        assert_eq!(files[0].name, file_data.name, "File name mismatch");
        env::remove_var("ASSETS_PATH");
    }

    #[test]
    fn test_remove_file() {
        let (test_file_path, _temp_dir, _files) = save_file();
        let file_id = 1;
        let remove_result = remove_file(file_id);
        assert!(remove_result.is_ok(), "Failed to delete file: {:?}", remove_result);
        let files = load_files_from_file().expect("Failed to load files");
        assert!(files.is_empty(), "File is not empty");
        env::remove_var("ASSETS_PATH");
    }

    #[test]
    fn test_get_file() {
        let (_test_file_path, _temp_dir, _files) = save_file();
        let file_id = 1;
        let get_result = get_file(file_id);
        assert!(get_result.is_ok(), "File was not found: {:?}", get_result);
        assert_eq!(get_result.unwrap().id, file_id, "File ID mismatch");
        env::remove_var("ASSETS_PATH");
    }

    #[test]
    fn test_modify_file() {
        let (test_file_path, _temp_dir, _files) = save_file();
        let file_id = 1;
        let mut updated_file = get_test_file();
        updated_file.id = 2;
        updated_file.name = String::from("updated-file");
        let modify_result = modify_file(file_id, updated_file.clone());
        assert!(modify_result.is_ok(), "File was not updated: {:?}", modify_result);
        let files = load_files_from_file().expect("Failed to load files");
        assert_eq!(files[0].name, "updated-file", "File name mismatch");
        env::remove_var("ASSETS_PATH");
    }
}