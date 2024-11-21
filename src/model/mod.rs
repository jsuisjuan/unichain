use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileType {
    Pdf,
    Docx,
    Xls,
    Txt,
    Csv,
    Pptx,
    Jpg,
    Png,
    Unknown
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub id: i64,
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
    pub created: NaiveDateTime,
    pub modified: Option<NaiveDateTime>,
    pub accessed: Option<NaiveDateTime>,
    pub owner: (i64, String, String),
    pub people_with_access: Vec<(i64, String, String)>,
    pub ipfs_hash: String,
    pub onchain_txn_id: String,
    pub download_permission: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileData {
    pub owner: (i64, String, String),
    pub name: String,
}