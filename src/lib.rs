use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use std::fs::File as StdFile;
use std::io::{Write, Read};
use bincode;
use idgenerator::*;
use std::process;

#[derive(Debug, Serialize, Deserialize)]
pub enum FileType {
    Pdf,
    Docx,
    Xls,
    Txt,
    Csv,
    Pptx,
    Jpg,
    Png,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: i64,
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
    pub created: NaiveDate,
    pub modified: NaiveDate,
    pub accessed: Option<NaiveDate>,
    pub owner: (i64, String, String),
    pub people_with_access: Vec<(i64, String, String)>,
    pub ipfs_hash: String,
    pub onchain_txn_id: String,
    pub download_permission: bool,
    pub description: Option<String>
}

pub fn generate_id() -> i64 {
    let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    let _ = IdInstance::init(options).unwrap_or_else(|err| {
        eprintln!("Problem on instancing id generator options: {err}");
        process::exit(1);
    });
    IdInstance::next_id()
}

pub fn load_files_from_file(path: &str) -> std::io::Result<Vec<File>> {
    let mut file = match StdFile::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(Vec::new())
    };
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let decoded: Vec<File> = bincode::deserialize(&buffer).expect("Failed to deserialize Vec<File>");
    Ok(decoded)
}

pub fn save_files_to_file(files: &Vec<File>, path: &str) -> std::io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(files).expect("Failed to serialize Vec<File>");
    let mut file = StdFile::create(path)?;
    file.write_all(&encoded)?;
    Ok(())
}

pub fn add_file(file: File, path: &str) -> std::io::Result<()> {
    let mut files = load_files_from_file(path)?;
    files.push(file);
    save_files_to_file(&files, path)
}

//Create
pub fn create_file() {

}

//Read
pub fn get_all_files() {

}

pub fn get_file() {

}

//Update
pub fn modify_file() {

}

//Delete
pub fn delete_file() {

}