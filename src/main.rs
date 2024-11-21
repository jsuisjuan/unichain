use std::{io::{self, Write}, path::Path};
use log::{info, error};
use anyhow::{Result, Error};

use unichain::{create_new_file, delete_file, get_all_files, get_file, modify_file, model::{File, FileData}, utils::generate_id};

const PATH: &str = "../assets/";

// things to do:
// - refact everything and move these functions to another file
// - docment every function
// - build unit test
// - test

fn get_system_owner() -> (i64, String, String) {
    (2454826096558341, String::from("Juan Carvalho Silva de Lima"), String::from("juanc.s.delima@gmail.com"))
}

fn print_menu_options() {
    println!("What do you want to do?\n");
    println!("1. View list of stored files\n2. View a specific file\n3. Store a new file\n4. Update an existing file\n5. Move a file to trash\n0. Exit");
}

fn get_choosed_option() -> Result<u8, String> {
    print!("Choose an option: ");
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    let mut choosed_option = String::new();
    io::stdin().read_line(&mut choosed_option).map_err(|_| "Failed to read from stdin")?;
    match choosed_option.trim().parse::<u8>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Please enter a valid number between 0 and 5.".to_string())
    }
}

fn get_files() -> Result<()> {
    info!("Fetching all the files.");
    let files: Vec<File> = match get_all_files(PATH) {
        Ok(files) => files,
        Err(e) => {
            error!("Failed to fetch files: {}", e);
            return Err(Error::msg(e));
        }
    };
    info!("Successfully fetched {} files.", files.len());
    println!("{:?}", files);
    Ok(())
}

fn get_specific_file() -> Result<(), String> {
    print!("Insert file ID: ");
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    let mut file_id_input: String = String::new();
    io::stdin().read_line(&mut file_id_input).map_err(|_| "Failed to read from stdin")?;
    let file_id: i64 = file_id_input.trim().parse::<i64>().map_err(|_| "Please enter a valid ID number.")?;
    let file: File = get_file(PATH, file_id)?;
    println!("File found\n{:?}", file);
    Ok(())
}

fn store_new_file(owner: (i64, String, String)) -> Result<(), String> {
    print!("Insert file path you want to store: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut file_path: String = String::new();
    io::stdin().read_line(&mut file_path).map_err(|e| e.to_string())?;
    let file_path: &str = file_path.trim();
    let path: &Path = Path::new(file_path);
    let filename: String = match path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => {
            eprintln!("Invalid file path");
            return Err("Invalid file path".to_string());
        }
    };
    print!("Your current file name is: {}. Do you want to change it? (Y/N): ", filename);
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut change_name_response: String = String::new();
    io::stdin().read_line(&mut change_name_response).map_err(|e| e.to_string())?;
    let change_name_response: String = change_name_response.trim().to_lowercase();
    let final_name: String = if change_name_response == "y" {
        print!("Enter the new file name: ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        let mut new_name: String = String::new();
        io::stdin().read_line(&mut new_name).map_err(|e| e.to_string())?;
        new_name.trim().to_string()
    } else {
        filename
    };
    let file_data = FileData {
        owner,
        name: final_name,
    };
    create_new_file(file_data, file_path, "PATH")?;
    Ok(())
}

fn update_existing_file() -> Result<(), String> {
    print!("Insert file ID: ");
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    let mut file_id_input: String = String::new();
    io::stdin().read_line(&mut file_id_input).map_err(|_| "Failed to read from stdin")?;
    let file_id: i64 = file_id_input.trim().parse::<i64>().map_err(|_| "Please enter a valid ID number.")?;
    let mut file: File = get_file(PATH, file_id)?;
    
    println!("\tModifying file");

    print!("Add new file name: ");
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    let mut new_name: String = String::new();
    io::stdin().read_line(&mut new_name).map_err(|_| "Failed to read from stdin")?;

    print!("Add new file description: ");
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    let mut new_description: String = String::new();
    io::stdin().read_line(&mut new_description).map_err(|_| "Failed to read from stdin")?;

    print!("Do you want to change people with access list?(Y/N): ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut response: String = String::new();
    io::stdin().read_line(&mut response).map_err(|e| e.to_string())?;
    let response: String = response.trim().to_lowercase();
    
    if response == "y" {
        loop {
            println!("\tEnter the new person information:");
            
            print!("Name: ");
            io::stdout().flush().map_err(|e| e.to_string())?;
            let mut person_name = String::new();
            io::stdin().read_line(&mut person_name).map_err(|e| e.to_string())?;
            let person_name = person_name.trim().to_string();
            
            print!("E-mail: ");
            io::stdout().flush().map_err(|e| e.to_string())?;
            let mut person_email = String::new();
            io::stdin().read_line(&mut person_email).map_err(|e| e.to_string())?;
            let person_email = person_email.trim().to_string();
    
            file.people_with_access.push((generate_id(), person_name, person_email));
            
            print!("Do you want to add another person? (Y/N): ");
            io::stdout().flush().map_err(|e| e.to_string())?;
            let mut add_another_person_response = String::new();
            io::stdin().read_line(&mut add_another_person_response).map_err(|e| e.to_string())?;
            let add_another_person_response = add_another_person_response.trim().to_lowercase();
    
            match add_another_person_response.as_str() {
                "y" => continue,
                "n" => break,
                _ => println!("Invalid input. Please enter 'Y' or 'N'.")
            }
        }
    }
    
    file.download_permission = loop {
        print!("Do you want to allow download permission for this file?(Y/N): ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        let mut download_permission_response = String::new();
        io::stdin().read_line(&mut download_permission_response).map_err(|e| e.to_string())?;
        download_permission_response = download_permission_response.trim().to_lowercase();
        match download_permission_response.as_str() {
            "y" => break true,
            "n" => break false,
            _ => println!("Invalid input. Please enter 'Y' or 'N'.")
        }
    };
    
    modify_file(PATH, file_id, file).map_err(|e| format!("Failed to modify file: {}", e))?;
    Ok(())
}

fn move_file_to_trash() -> Result<(), String> {
    print!("Insert file ID: ");
    io::stdout().flush().map_err(|_| "Failed to flush stdout")?;
    let mut file_id_input: String = String::new();
    io::stdin().read_line(&mut file_id_input).map_err(|_| "Failed to read from stdin")?;
    let file_id: i64 = file_id_input.trim().parse::<i64>().map_err(|_| "Please enter a valid ID number.")?;
    delete_file(PATH, file_id)?;
    println!("File ID {} was moved to the trash!", file_id);
    Ok(())
}

fn system_menu(owner: (i64, String, String)) -> Result<(), String> {
    print_menu_options();
    loop {
        match get_choosed_option() {
            Ok(option) if (0..=5).contains(&option) => {
                match option {
                    0 => {println!("Exiting..."); break Ok(());},
                    1 => {let _ = get_files(); break Ok(());},
                    2 => {get_specific_file()?; break Ok(());},
                    3 => {store_new_file(owner)?; break Ok(());},
                    4 => {update_existing_file()?; break Ok(());},
                    5 => {move_file_to_trash()?; break Ok(());},
                    _ => unreachable!(),
                };
            },
            _ => println!("Please enter a valid number between 0 and 5.")
        }
    }
}

fn main() {
    let owner: (i64, String, String) = get_system_owner();
    println!("\t\tWelcome to your UniChain!\nusername: {}\ne-mail: {} ", owner.1, owner.2);
    let _ = system_menu(owner);
}


