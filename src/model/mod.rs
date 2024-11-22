use chrono::NaiveDateTime;
use std::{io, fmt};
use serde::{Serialize, Deserialize};
use std::error::Error;


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

// Implement Display for FileError
impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::InputError(msg) => write!(f, "Input error: {}", msg),
            FileError::ParseError => write!(f, "Invalid file ID input, please enter a valid number."),
            FileError::FileNotFound => write!(f, "File not found."),
            FileError::IOError(err) => write!(f, "I/O error: {}", err),
            FileError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            FileError::FileAlreadyExists => write!(f, "The file already exists."),
            FileError::PermissionDenied => write!(f, "Permission denied."),
            FileError::IdGenerationError(msg) => write!(f, "ID generation error: {}", msg),
            FileError::InvalidFileType(msg) => write!(f, "Invalid file type: {}", msg),
            FileError::InvalidFileSize => write!(f, "Invalid file size."),
        }
    }
}

// Implement the Error trait for FileError
impl Error for FileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileError::IOError(err) => Some(err), // `IOError` is already an `io::Error`
            _ => None,
        }
    }
}

// Implement `From<io::Error>` for `FileError` to simplify conversions
impl From<io::Error> for FileError {
    fn from(error: io::Error) -> Self {
        FileError::IOError(error)
    }
}
