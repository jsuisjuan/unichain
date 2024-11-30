use std::io::{self, Write};
use log::{info, warn};

use crate::{get_file, modify_file};
use crate::model::{File, FileError};
use crate::utils::{process_input, generate_id, prompt_for_file_id};

pub fn update_file() -> Result<(), FileError> {
    loop {
        let file_id = prompt_for_file_id()?;
        let mut file = fetch_and_validate_file(file_id)?;
        update_file_attributes(&mut file)?;
        modify_file(file_id, file).map_err(|e| FileError::InputError(e.to_string()))?;
        return Ok(());
    }
}

fn fetch_and_validate_file(file_id: i64) -> Result<File, FileError> {
    match get_file(file_id) {
        Ok(file) => {
            info!("Modifying file with ID: {}", file_id);
            Ok(file)
        },
        Err(_) => {
            print!("\n");
            warn!("File not found.");
            Err(FileError::FileNotFound)
        }
    }
}

fn update_file_attributes(file: &mut File) -> Result<(), FileError> {
    file.name = process_input("Add new file name: ", false)?.unwrap();
    file.description = process_input("Add new file description: ", true)?;
    if ask_yes_no("Do you want to change the people with access list? (Y/N): ")? {
        update_people_with_access(file)?;
    }
    file.download_permission = ask_yes_no("Do you want to allow download permission for this file? (Y/N): ")?;
    Ok(())
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