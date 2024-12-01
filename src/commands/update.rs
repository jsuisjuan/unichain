use log::{info, warn};

use crate::{get_file, modify_file};
use crate::model::{File, FileError};
use crate::utils::{process_input, generate_id, prompt_for_file_id, handle_input};

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
    get_file(file_id)
        .map(|file| { info!("Modifying file with ID: {}", file_id); file })
        .or_else(|_| { warn!("File not found."); Err(FileError::FileNotFound) })
}

fn update_file_attributes(file: &mut File) -> Result<(), FileError> {
    file.name = process_input("Add new file name: ", false)?.expect("Name should not be empty");
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
        match handle_input()?.trim().to_lowercase().as_str() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => println!("Invalid input. Please enter 'Y' or 'N'.")
        }
    }
}

fn update_people_with_access(file: &mut File) -> Result<(), FileError> {
    loop {
        println!("\tEnter the new person information:");
        let name = process_input("Name: ", false)?.expect("Name should not be empty");
        let email = process_input("E-mail: ", false)?.expect("Email should not be empty");
        file.people_with_access.push((generate_id()?, name, email));
        if !ask_yes_no("Do you want to add another person? (Y/N): ")? {
            break;
        }
    }
    Ok(())
}