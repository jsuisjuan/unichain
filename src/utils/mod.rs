use std::path::{Path, PathBuf};
use std::io::{self, Write};

use chrono::Utc;
use idgenerator::*;
use rand::{distributions::Alphanumeric, Rng};
use log::warn;

use crate::model::{File, FileType, FileData, FileError};

pub fn generate_id() -> Result<i64, FileError> {
    let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    IdInstance::init(options).map_err(|err| FileError::IdGenerationError(format!("Error initializing ID generator: {}", err)))?;
    Ok(IdInstance::next_id())
}

pub fn generate_fake_hash(length: usize) -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

pub fn get_file_type(file_path: &PathBuf) -> Result<FileType, FileError> {
    let path = Path::new(file_path);
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("pdf") => Ok(FileType::Pdf),
        Some("docx") => Ok(FileType::Docx),
        Some("xls") => Ok(FileType::Xls),
        Some("txt") => Ok(FileType::Txt),
        Some("csv") => Ok(FileType::Csv),
        Some("pptx") => Ok(FileType::Pptx),
        Some("jpg") => Ok(FileType::Jpg),
        Some("png") => Ok(FileType::Png),
        Some(ext) => Err(FileError::InvalidFileType(ext.to_string())),
        None => Err(FileError::InvalidFileType("Unknown extension".to_string())),
    }
}

pub fn get_file_size(file_path: &PathBuf) -> Result<u64, FileError> {
    match std::fs::metadata(file_path) {
        Ok(metadata) => Ok(metadata.len()),
        Err(err) => Err(FileError::IOError(err))
    }
}

pub fn get_default_file(file_data: &FileData, path: &PathBuf) -> Result<File, FileError> {
    let owner_access = file_data.owner.clone();
    let file_size = get_file_size(path).map_err(|e| e)?; 
    Ok(File {
        id: generate_id()?,
        name: String::new(),
        file_type: get_file_type(path)?,
        size: file_size,
        created: Utc::now().naive_utc(),
        modified: None,
        accessed: None,
        owner: file_data.owner.clone(),
        people_with_access: vec![owner_access],
        ipfs_hash: generate_fake_hash(46),
        onchain_txn_id: generate_fake_hash(64),
        download_permission: false,
        description: None,
    })
}

pub fn process_modified_file(mut file: File) -> Result<File, FileError> {
    file.modified = Some(Utc::now().naive_utc());
    file.accessed = Some(Utc::now().naive_utc());
    Ok(file)
}

pub fn update_accessed_file_date(mut file: File) -> Result<File, FileError> {
    file.accessed = Some(Utc::now().naive_utc());
    Ok(file)
}

pub fn process_input(prompt: &str, allow_empty: bool) -> Result<Option<String>, FileError> {
    loop {
        print!("{}", prompt);
        io::stdout().flush().map_err(FileError::IOError)?;
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(FileError::IOError)?;
        let trimmed_input = input.trim();
        if trimmed_input.is_empty() && !allow_empty {
            warn!("Input cannot be empty for prompt '{}'. Please try again.", prompt);
            continue;
        }
        return Ok(if trimmed_input.is_empty() { None } else { Some(trimmed_input.to_string()) });
    }
}

pub fn get_system_owner() -> (i64, String, String) {
    (2454826096558341, String::from("Juan Carvalho Silva de Lima"), String::from("juanc.s.delima@gmail.com"))
}

pub fn prompt_for_file_id() -> Result<i64, FileError> {
    loop {
        print!("\nInsert file ID: ");
        io::stdout().flush().map_err(|e| FileError::IOError(e))?;
        let mut file_id_input = String::new();
        io::stdin().read_line(&mut file_id_input).map_err(|e| FileError::IOError(e))?;
        match file_id_input.trim().parse::<i64>() {
            Ok(file_id) => return Ok(file_id),
            Err(_) => {
                print!("\n");
                warn!("Invalid ID number. Please enter a valid number.");
                continue;
            }
        }
    }
}

pub fn handle_input() -> Result<String, FileError> {
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut response = String::new();
    io::stdin().read_line(&mut response).map_err(|e| FileError::IOError(e))?;
    Ok(response.trim().to_string())
}