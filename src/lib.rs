use std::fs::File as StdFile;
use std::io::{Read, Write};
use bincode;

mod model;
use model::File;

mod utils;
use utils::{get_default_file, FileData};


fn load_files_from_file(path: &str) -> std::io::Result<Vec<File>> {
    let mut file: StdFile = match StdFile::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(Vec::new()),
    };
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;
    let decoded: Vec<File> = bincode::deserialize(&buffer).expect("Failed to deserialize Vec<File>");
    Ok(decoded)
}

fn save_files_to_file(files: &Vec<File>, path: &str) -> std::io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(files).expect("Vec<File> serialization failed");
    let mut file: StdFile = StdFile::create(path)?;
    file.write_all(&encoded)?;
    Ok(())
}

fn add_files_to_file(file: File, path: &str) -> std::io::Result<()> {
    let mut files: Vec<File> = load_files_from_file(path)?;
    files.push(file);
    save_files_to_file(&files, path)
}

pub fn create_new_file(file_data: FileData, path: &str) -> Result<(), String> {
    let mut file: File = get_default_file(&file_data, path).map_err(|e| format!("Erro ao criar arquivo: {}", e))?;
    file.name = file_data.name;
    add_files_to_file(file, path).map_err(|e| format!("Error saving file: {}", e))?;
    println!("File created and saved successfully");
    Ok(())
}

pub fn get_all_files(path: &str) -> Result<Vec<File>, String> {
    load_files_from_file(path).map_err(|e| format!("Error loading file: {}", e))
}

pub fn get_file(path: &str, file_id: i64) -> Result<File, String> {
    let files: Vec<File> = get_all_files(path)?;
    files.into_iter().find(|file| file.id == file_id).ok_or_else(|| "File not found".to_string())
}

pub fn modify_file(path: &str, file_id: i64, updated_file: File) -> Result<(), String> {
    let mut files: Vec<File> = get_all_files(path)?;
    let file_index: usize = files.iter().position(|file| file.id == file_id).ok_or_else(|| "File not found".to_string())?;
    files[file_index] = updated_file;
    save_files_to_file(&files, path).map_err(|e| format!("Error updating file: {}", e))
}

pub fn delete_file(path: &str, file_id: i64) -> Result<(), String> {
    let mut files: Vec<File> = get_all_files(path)?;
    let initial_length: usize = files.len();
    files.retain(|file| file.id != file_id);
    if files.len() == initial_length {
        return Err("File not found".to_string());
    }
    save_files_to_file(&files, path).map_err(|e| format!("Error updating file: {}", e))
}