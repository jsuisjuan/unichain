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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use chrono::Utc;
    use crate::model::{File, FileType, FileData, FileError};

    #[test]
    fn test_generate_id_success() {
        let result = generate_id();
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);  // Assuming a positive ID is generated
    }

    #[test]
    fn test_generate_fake_hash() {
        let hash = generate_fake_hash(16);
        assert_eq!(hash.len(), 16);  // Test if generated hash has correct length
    }

    #[test]
    fn test_get_file_type_valid_pdf() {
        let file_path = PathBuf::from("file.pdf");
        let result = get_file_type(&file_path);
        assert_eq!(result, Ok(FileType::Pdf));
    }

    #[test]
    fn test_get_file_type_invalid_extension() {
        let file_path = PathBuf::from("file.xyz");
        let result = get_file_type(&file_path);
        assert_eq!(result, Err(FileError::InvalidFileType("xyz".to_string())));
    }

    #[test]
    fn test_get_file_size_valid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();
        let result = get_file_size(&file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);  // File is empty, so size should be 0
    }

    #[test]
    fn test_get_file_size_invalid() {
        let file_path = PathBuf::from("/invalid/path/to/file.txt");
        let result = get_file_size(&file_path);
        assert!(result.is_err());  // File should not exist
    }

    #[test]
    fn test_get_default_file_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let file_data = FileData {
            owner: String::from("Juan Carvalho Silva de Lima"),
        };

        let result = get_default_file(&file_data, &file_path);
        assert!(result.is_ok());

        let file = result.unwrap();
        assert_eq!(file.owner, file_data.owner);
        assert_eq!(file.size, 0);  // File is empty
    }

    #[test]
    fn test_process_modified_file() {
        let file = File {
            id: 1,
            name: String::from("file.txt"),
            file_type: FileType::Txt,
            size: 100,
            created: Utc::now().naive_utc(),
            modified: None,
            accessed: None,
            owner: String::from("Juan"),
            people_with_access: vec![],
            ipfs_hash: String::from("hash"),
            onchain_txn_id: String::from("txn_id"),
            download_permission: false,
            description: None,
        };

        let result = process_modified_file(file);
        assert!(result.is_ok());
        let updated_file = result.unwrap();
        assert!(updated_file.modified.is_some());
        assert!(updated_file.accessed.is_some());
    }

    #[test]
    fn test_update_accessed_file_date() {
        let file = File {
            id: 1,
            name: String::from("file.txt"),
            file_type: FileType::Txt,
            size: 100,
            created: Utc::now().naive_utc(),
            modified: None,
            accessed: None,
            owner: String::from("Juan"),
            people_with_access: vec![],
            ipfs_hash: String::from("hash"),
            onchain_txn_id: String::from("txn_id"),
            download_permission: false,
            description: None,
        };

        let result = update_accessed_file_date(file);
        assert!(result.is_ok());
        let updated_file = result.unwrap();
        assert!(updated_file.accessed.is_some());
    }

    #[test]
    fn test_process_input_non_empty_success() {
        // Simulate user input for a non-empty prompt
        let user_input = "Test Input".to_string();
        let result = process_input("Enter something: ", false);
        assert_eq!(result, Ok(Some(user_input)));
    }

    #[test]
    fn test_process_input_empty_success() {
        // Simulate user input for an empty prompt
        let result = process_input("Enter something: ", true);
        assert_eq!(result, Ok(None));  // Allowing empty input
    }

    #[test]
    fn test_process_input_empty_fail() {
        // Simulate empty input when not allowed
        let result = process_input("Enter something: ", false);
        assert!(result.is_err());  // Expected to fail since input is empty and not allowed
    }

    #[test]
    fn test_get_system_owner() {
        let result = get_system_owner();
        assert_eq!(result.0, 2454826096558341);
        assert_eq!(result.1, "Juan Carvalho Silva de Lima");
        assert_eq!(result.2, "juanc.s.delima@gmail.com");
    }

    #[test]
    fn test_prompt_for_file_id_success() {
        // Simulate valid user input for file ID
        let user_input = "12345".to_string();
        let result = prompt_for_file_id();
        assert_eq!(result, Ok(12345));
    }

    #[test]
    fn test_prompt_for_file_id_invalid() {
        // Simulate invalid user input for file ID
        let user_input = "abc".to_string();
        let result = prompt_for_file_id();
        assert!(result.is_err());  // Invalid input
    }
}
