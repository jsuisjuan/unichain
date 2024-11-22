use std::path::{Path, PathBuf};

use chrono::Utc;
use idgenerator::*;
use rand::{distributions::Alphanumeric, Rng};
use log::error;

use crate::model::{File, FileType, FileData, FileError};

pub fn generate_id() -> Result<i64, FileError> {
    let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    if let Err(err) = IdInstance::init(options) {
        error!("Error initializing ID generator: {}", err);
        return Err(FileError::IdGenerationError(err.to_string()));
    }
    Ok(IdInstance::next_id())
}

fn generate_fake_hash(length: usize) -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

pub fn get_file_type(file_path: &PathBuf) -> Result<FileType, FileError> {
    let path: &Path = Path::new(file_path);
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
    let owner_access: (i64, String, String) = file_data.owner.clone();
    let file_size: u64 = get_file_size(path).map_err(|e| e)?; 
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

fn process_input(prompt: &str, allow_empty: bool) -> Result<Option<String>, FileError> {
    print!("{}", prompt);
    io::stdout().flush().map_err(FileError::IOError)?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(FileError::IOError)?;
    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        if allow_empty {
            return Ok(None);
        } else {
            error!("Input cannot be empty for prompt '{}'.", prompt);
            return Err(FileError::InputError("Input cannot be empty.".to_string()));
        }
    }
    Ok(Some(trimmed_input.to_string()))
}

fn get_system_owner() -> (i64, String, String) {
    (2454826096558341, String::from("Juan Carvalho Silva de Lima"), String::from("juanc.s.delima@gmail.com"))
}