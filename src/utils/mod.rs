use std::path::PathBuf;
use std::io::{self, Write};

use chrono::{NaiveDate, NaiveDateTime, Utc};
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

pub fn get_file_type_from_input() -> Result<FileType, FileError> {
    print!("Enter the file type (pdf, docx, xls, txt, csv, pptx, jpg, png): ");
    io::stdout().flush().map_err(|_| FileError::IOError(std::io::Error::new(io::ErrorKind::Other, "Failed to flush output")))?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| FileError::IOError(std::io::Error::new(io::ErrorKind::Other, "Failed to read input")))?;
    let file_type = input.trim().to_lowercase();
    match file_type.as_str() {
        "pdf" => Ok(FileType::Pdf),
        "docx" => Ok(FileType::Docx),
        "xls" => Ok(FileType::Xls),
        "txt" => Ok(FileType::Txt),
        "csv" => Ok(FileType::Csv),
        "pptx" => Ok(FileType::Pptx),
        "jpg" => Ok(FileType::Jpg),
        "png" => Ok(FileType::Png),
        _ => Err(FileError::InvalidFileType(format!("Unsupported file type: {}", file_type))),
    }
}


pub fn get_file_size() -> Result<u64, FileError> {
    print!("Insert a size (in bytes) to allocate the file: ");
    io::stdout().flush().map_err(FileError::IOError)?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(FileError::IOError)?;
    let size = input.trim().parse::<u64>().map_err(|_| FileError::ParseError)?;
    if size == 0 {
        return Err(FileError::InputError("Size cannot be zero.".to_string()));
    }
    Ok(size)
}

pub fn parse_date_input() -> Result<NaiveDateTime, String> {
    print!("Enter the creation date (DD/MM/YYYY): ");
    io::stdout().flush().map_err(|_| "Failed to flush output".to_string())?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| "Failed to read input".to_string())?;
    let input = input.trim();
    let date_parts: Vec<&str> = input.split('/').collect();
    if date_parts.len() != 3 {
        return Err("Invalid date format. Please use DD/MM/YYYY.".to_string());
    }
    let day: u32 = date_parts[0].parse().map_err(|_| "Invalid day.".to_string())?;
    let month: u32 = date_parts[1].parse().map_err(|_| "Invalid month.".to_string())?;
    let year: i32 = date_parts[2].parse().map_err(|_| "Invalid year.".to_string())?;
    let date = NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date.")?;
    let now = Utc::now().naive_utc();
    let created_time = NaiveDateTime::new(date, now.time());
    Ok(created_time)
}


pub fn get_default_file(file_data: &FileData, _path: &PathBuf) -> Result<File, FileError> {
    let owner_access = file_data.owner.clone();
    let file_size = get_file_size().map_err(|e| e)?;
    let created_date = parse_date_input().unwrap();
    let file_type = get_file_type_from_input()?;
    Ok(File {
        id: generate_id()?,
        name: String::new(),
        file_type: file_type,
        size: file_size,
        created: created_date,
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
    file.size = get_file_size().map_err(|e| e)?;
    file.file_type = get_file_type_from_input()?;
    file.modified = Some(parse_date_input().unwrap());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_fake_hash() {
        let hash = generate_fake_hash(10);
        assert_eq!(hash.len(), 10);
        assert!(hash.chars().all(|c| c.is_ascii_alphanumeric()));
    }  
}