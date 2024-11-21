use std::fs::{metadata, Metadata};
use std::path::{Path, PathBuf};
use std::process;

use chrono::Utc;
use idgenerator::*;
use rand::{distributions::Alphanumeric, Rng};

use crate::model::{File, FileType, FileData};

pub fn generate_id() -> i64 {
    let options: IdGeneratorOptions = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    let _ = IdInstance::init(options).unwrap_or_else(|err| {
        eprintln!("Error initializing ID generator: {err}");
        process::exit(1);
    });
    IdInstance::next_id()
}

fn generate_fake_hash(length: usize) -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

pub fn get_file_type(file_path: &PathBuf) -> FileType {
    let path: &Path = Path::new(file_path);
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("pdf") => FileType::Pdf,
        Some("docx") => FileType::Docx,
        Some("xls") => FileType::Xls,
        Some("txt") => FileType::Txt,
        Some("csv") => FileType::Csv,
        Some("pptx") => FileType::Pptx,
        Some("jpg") => FileType::Jpg,
        Some("png") => FileType::Png,
        _ => FileType::Unknown,
    }
}

pub fn get_file_size(file_path: &PathBuf) -> std::io::Result<u64> {
    let metadata: Metadata = metadata(file_path)?;
    Ok(metadata.len())
}

pub fn get_default_file(file_data: &FileData, path: &PathBuf) -> Result<File, String> {
    let owner_access: (i64, String, String) = file_data.owner.clone();
    let file_size: u64 = get_file_size(path).map_err(|e| e.to_string())?; 
    Ok(File {
        id: generate_id(),
        name: String::new(),
        file_type: get_file_type(path),
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

pub fn process_modified_file(mut file: File) -> Result<File, String> {
    file.modified = Some(Utc::now().naive_utc());
    file.accessed = Some(Utc::now().naive_utc());
    Ok(file)
}

pub fn update_accessed_file_date(mut file: File) -> Result<File, String> {
    file.accessed = Some(Utc::now().naive_utc());
    Ok(file)
}
