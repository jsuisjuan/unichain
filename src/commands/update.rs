use std::io::{self, Write};
use log::{info, warn};

use crate::{get_file, modify_file};
use crate::model::{File, FileError};
use crate::utils::{process_input, generate_id, prompt_for_file_id};

pub fn update_file() -> Result<(), FileError> {
    loop {
        let file_id = prompt_for_file_id()?;
        let mut file = match get_file(file_id) {
            Ok(file) => file,
            Err(_) => {
                print!("\n");
                warn!("File not found.");
                continue;
            }
        };
        info!("Modifying file with ID: {}", file_id);
        file.name = process_input("Add new file name: ", false)?.unwrap();
        file.description = process_input("Add new file description: ", true)?;
        if ask_yes_no("Do you want to change the people with access list? (Y/N): ")? {
            update_people_with_access(&mut file)?;
        }
        file.download_permission = ask_yes_no("Do you want to allow download permission for this file? (Y/N): ")?;
        modify_file(file_id, file).map_err(|e| FileError::InputError(e.to_string()))?;
        return Ok(());
    }
}

fn ask_yes_no(prompt: &str) -> Result<bool, FileError> {
    loop {
        print!("{}", prompt);
        io::stdout().flush().map_err(|e| FileError::IOError(e))?;
        let mut response = String::new();
        io::stdin().read_line(&mut response).map_err(|e| FileError::IOError(e))?;
        match response.trim().to_lowercase().as_str() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => println!("Invalid input. Please enter 'Y' or 'N'.")
        }
    }
}

fn update_people_with_access(file: &mut File) -> Result<(), FileError> {
    loop {
        println!("\tEnter the new person information:");
        let name = process_input("Name: ", false)?.unwrap();
        let email = process_input("E-mail: ", false)?.unwrap();
        file.people_with_access.push((generate_id()?, name, email));
        if !ask_yes_no("Do you want to add another person? (Y/N): ")? {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::model::{File, FileType};
    use crate::utils::generate_fake_hash;
 
    fn mock_ask_yes_no(input: &str) -> Result<bool, String> {
        match input.to_string().trim().to_lowercase().as_str() {
            "y" => Ok(true),
            "n" => Ok(false),
            _ => Err("Invalid input. Please enter 'Y' or 'N'.".to_string()),
        }
    }

    fn get_test_file() -> File {
        let owner = (1, String::from("Username"), String::from("username@gmail.com"));
        let owner_access = owner.clone();
        File { 
            id: 1, name: "test-file".to_string(), file_type: FileType::Pdf, size: 100, 
            created: Utc::now().naive_utc(), modified: None, accessed: None, owner, 
            people_with_access: vec![owner_access], ipfs_hash: generate_fake_hash(46), 
            onchain_txn_id: generate_fake_hash(64), download_permission: false, description: None 
        }
    }

    #[test]
    fn test_ask_yes_no_valid_input() {
        let result = mock_ask_yes_no("Y").unwrap();
        assert_eq!(result, true);
        let result = mock_ask_yes_no("N").unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_ask_yes_no_invalid_input() {
        let result = mock_ask_yes_no("Invalid");
        assert_eq!(result, Err("Invalid input. Please enter 'Y' or 'N'.".to_string()));
    }

    #[test]
    fn test_update_people_with_access() {
        let mut file = get_test_file();
        file.people_with_access = vec![];
        let result = update_people_with_access(&mut file);
        assert!(result.is_ok());
        assert_eq!(file.people_with_access.len(), 1);
        assert_eq!(file.people_with_access[0].1, "Test Name");
        assert_eq!(file.people_with_access[0].2, "test@example.com");
    }
}
