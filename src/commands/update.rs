use std::io::{self, Write};
use log::info;

use crate::{get_file, modify_file};
use crate::model::{File, FileError};
use crate::utils::{process_input, generate_id};

const PATH: &str = "../assets";

pub fn update_file() -> Result<(), FileError> {
    let file_id = prompt_for_file_id()?;
    let mut file = get_file(PATH, file_id).map_err(|e| FileError::InputError(e.to_string()))?;
    info!("Modifying file with ID: {}", file_id);
    file.name = process_input("Add new file name: ", false)?.unwrap();
    file.description = process_input("Add new file description: ", true)?;
    if ask_yes_no("Do you want to change the people with access list? (Y/N): ")? {
        update_people_with_access(&mut file)?;
    }
    file.download_permission = ask_yes_no("Do you want to allow download permission for this file? (Y/N): ")?;
    modify_file(PATH, file_id, file).map_err(|e| FileError::InputError(e.to_string()))?;
    Ok(())
}

fn prompt_for_file_id() -> Result<i64, FileError> {
    print!("Insert file ID: ");
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut file_id_input = String::new();
    io::stdin().read_line(&mut file_id_input).map_err(|e| FileError::IOError(e))?;
    let file_id = file_id_input.trim().parse::<i64>().map_err(|_| FileError::InputError("Invalid ID number.".to_string()))?;
    Ok(file_id)
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