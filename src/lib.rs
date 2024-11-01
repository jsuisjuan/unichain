use chrono::NaiveDate;
use idgenerator::*;
use std::process;

#[derive(Debug)]
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

#[derive(Debug)]
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