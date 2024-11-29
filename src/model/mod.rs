use std::error::Error;
use std::{io, fmt};

use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct File {
    pub id: i64,
    pub name: String,
    pub file_type: FileType, //talvez altera
    pub size: u64, //altera
    pub created: NaiveDateTime, //altera
    pub modified: Option<NaiveDateTime>,
    pub accessed: Option<NaiveDateTime>,
    pub owner: (i64, String, String),
    pub people_with_access: Vec<(i64, String, String)>,
    pub ipfs_hash: String,
    pub onchain_txn_id: String,
    pub download_permission: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileData {
    pub owner: (i64, String, String),
    pub name: String,
}

#[derive(Debug)]
pub enum FileError {
    InputError(String),
    ParseError,
    FileNotFound,
    IOError(io::Error),
    DeserializationError(String),
    FileAlreadyExists,
    PermissionDenied,
    IdGenerationError(String),
    InvalidFileType(String),
    InvalidFileSize,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::InputError(msg) => write!(f, "Input error :: {}", msg),
            FileError::ParseError => write!(f, "Invalid file ID input, please enter a valid number."),
            FileError::FileNotFound => write!(f, "File not found."),
            FileError::IOError(err) => write!(f, "I/O error :: {}", err),
            FileError::DeserializationError(msg) => write!(f, "Deserialization error :: {}", msg),
            FileError::FileAlreadyExists => write!(f, "The file already exists."),
            FileError::PermissionDenied => write!(f, "Permission denied."),
            FileError::IdGenerationError(msg) => write!(f, "ID generation error :: {}", msg),
            FileError::InvalidFileType(msg) => write!(f, "Invalid file type :: {}", msg),
            FileError::InvalidFileSize => write!(f, "Invalid file size."),
        }
    }
}

impl PartialEq for FileError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileError::InputError(a), FileError::InputError(b)) => a == b,
            (FileError::ParseError, FileError::ParseError) => true,
            (FileError::FileNotFound, FileError::FileNotFound) => true,
            (FileError::IOError(a), FileError::IOError(b)) => a.kind() == b.kind(),
            (FileError::DeserializationError(a), FileError::DeserializationError(b)) => a == b,
            (FileError::FileAlreadyExists, FileError::FileAlreadyExists) => true,
            (FileError::PermissionDenied, FileError::PermissionDenied) => true,
            (FileError::IdGenerationError(a), FileError::IdGenerationError(b)) => a == b,
            (FileError::InvalidFileType(a), FileError::InvalidFileType(b)) => a == b,
            (FileError::InvalidFileSize, FileError::InvalidFileSize) => true,
            _ => false,
        }
    }
}


impl Error for FileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileError::IOError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for FileError {
    fn from(error: io::Error) -> Self {
        FileError::IOError(error)
    }
}
