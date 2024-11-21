use std::{io::{self, Write}, path::PathBuf};
use log::{info, error};
use anyhow::Result;

use unichain::{create_new_file, delete_file, get_all_files, get_file, modify_file, model::{File, FileData, FileError}, utils::generate_id};

const PATH: &str = "../assets/";

fn get_system_owner() -> (i64, String, String) {
    (2454826096558341, String::from("Juan Carvalho Silva de Lima"), String::from("juanc.s.delima@gmail.com"))
}

fn print_menu_options() {
    println!("What do you want to do?\n");
    println!("1. View list of stored files\n2. View a specific file\n3. Store a new file\n4. Update an existing file\n5. Move a file to trash\n0. Exit");
}

fn get_choosed_option() -> Result<u8, FileError> {
    print!("Choose an option (0-5): ");
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut choosed_option = String::new();
    io::stdin().read_line(&mut choosed_option).map_err(|e| FileError::IOError(e))?;
    match choosed_option.trim().parse::<u8>() {
        Ok(num) if (0..=5).contains(&num) => Ok(num),
        Ok(_) => Err(FileError::InputError("The number must be between 0 and 5.".to_string())),
        Err(e) => Err(FileError::InputError(format!("Invalid input, please enter a number: {}", e)))
    }
}

fn get_files() -> Result<(), FileError> {
    info!("Fetching all the files.");
    let files: Vec<File> = match get_all_files(PATH) {
        Ok(files) => files,
        Err(e) => {
            error!("Failed to fetch files: {}", e);
            return Err(FileError::FileNotFound);
        }
    };
    info!("Successfully fetched {} files.", files.len());
    println!("{:?}", files);
    Ok(())
}

fn get_specific_file() -> Result<(), FileError> {
    print!("Insert file ID: ");
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut file_id_input = String::new();
    io::stdin().read_line(&mut file_id_input).map_err(|e| FileError::IOError(e))?;
    let file_id = file_id_input.trim().parse::<i64>().map_err(|_| FileError::ParseError)?;
    let file = get_file(PATH, file_id).map_err(|_| FileError::FileNotFound)?;
    println!("File found\n{:?}", file);
    Ok(())
}

fn prompt_for_file_path() -> Result<PathBuf, FileError> {
    print!("Insert file path you want to store: ");
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).map_err(|e| FileError::IOError(e))?;
    let trimmed_path = file_path.trim();
    if trimmed_path.is_empty() {
        error!("File path input is empty.");
        return Err(FileError::InputError("File path cannot be empty.".to_string()));
    }
    Ok(PathBuf::from(trimmed_path))
}

fn extract_filename(path: &PathBuf) -> Result<PathBuf, FileError> {
    match path.file_name() {
        Some(name) => Ok(name.to_string_lossy().into_owned().into()),
        None => {
            error!("Invalid file path: {}", path.display());
            Err(FileError::InputError("Invalid file path".to_string()))
        }
    }
}

fn ask_for_filename_change(current_name: &PathBuf) -> Result<String, FileError> {
    print!("Your current file name is: {}. Do you want to change it? (Y/N): ", current_name.display());
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut response = String::new();
    io::stdin().read_line(&mut response).map_err(|e| FileError::IOError(e))?;
    let trimmed_response = response.trim().to_lowercase();
    if trimmed_response == "y" {
        prompt_for_new_file_name()
    } else {
        Ok(current_name.to_string_lossy().into_owned())
    }
}

fn prompt_for_new_file_name() -> Result<String, FileError> {
    print!("Enter the new file name: ");
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut new_name = String::new();
    io::stdin().read_line(&mut new_name).map_err(|e| FileError::IOError(e))?;
    let trimmed_name = new_name.trim();
    if trimmed_name.is_empty() {
        error!("New file name input is empty.");
        FileError::InputError("New file name cannot be empty.".to_string());
    }
    Ok(trimmed_name.to_string())
}

fn prompt_for_new_description() -> Result<Option<String>, FileError> {
    print!("Add new file description:" );
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut new_description = String::new();
    io::stdin().read_line(&mut new_description).map_err(|e| FileError::IOError(e))?;
    let trimmed_description = new_description.trim();
    Ok(if trimmed_description.is_empty() { None } else { Some(trimmed_description.to_string())})
}

fn store_new_file(owner: (i64, String, String)) -> Result<(), FileError> {
    let file_path= prompt_for_file_path()?;
    let filename = extract_filename(&file_path)?;
    let final_name = ask_for_filename_change(&filename)?;
    let file_data = FileData {
        owner,
        name: final_name,
    };
    let _ = create_new_file(file_data, &file_path, "PATH");
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

fn prompt_for_input(prompt: &str) -> Result<String, FileError> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| FileError::IOError(e))?;
    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        error!("Input cannot be empty for prompt '{}'.", prompt);
        FileError::InputError("Input cannot be empty.".to_string());
    }
    Ok(trimmed_input.to_string())
}

fn update_people_with_access(file: &mut File) -> Result<(), FileError> {
    loop {
        println!("\tEnter the new person information:");
        let person_name = prompt_for_input("Name: ")?;
        let person_email = prompt_for_input("E-mail: ")?;
        file.people_with_access.push((generate_id(), person_name, person_email));
        if !ask_yes_no("Do you want to add another person? (Y/N): ")? {
            break;
        }
    }
    Ok(())
}

fn update_existing_file() -> Result<(), FileError> {
    let file_id = prompt_for_file_id()?;
    let mut file = get_file(PATH, file_id).map_err(|e| FileError::InputError(e))?;
    info!("Modifying file with ID: {}", file_id);
    file.name = prompt_for_new_file_name()?;
    file.description = prompt_for_new_description()?;
    if ask_yes_no("Do you want to change the people with access list? (Y/N): ")? {
        update_people_with_access(&mut file)?;
    }
    file.download_permission = ask_yes_no("Do you want to allow download permission for this file? (Y/N): ")?;
    modify_file(PATH, file_id, file).map_err(|e| FileError::InputError(e))?;
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

fn move_file_to_trash() -> Result<(), FileError> {
    print!("Insert file ID: ");
    io::stdout().flush().map_err(|e| FileError::IOError(e))?;
    let mut file_id_input = String::new();
    io::stdin().read_line(&mut file_id_input).map_err(|e| FileError::IOError(e))?;
    let file_id: i64 = file_id_input.trim().parse::<i64>().map_err(|_| FileError::ParseError)?;
    let _ = delete_file(PATH, file_id);
    println!("File ID {} was moved to the trash!", file_id);
    Ok(())
}

fn system_menu(owner: (i64, String, String)) -> Result<(), FileError> {
    print_menu_options();
    loop {
        match get_choosed_option()? {
            option if (0..=5).contains(&option) => {
                match option {
                    0 => {
                        println!("Exiting...");
                        break Ok(());
                    },
                    1 => {
                        get_files()?;
                        break Ok(());
                    },
                    2 => {
                        get_specific_file()?;
                        break Ok(());
                    },
                    3 => {
                        store_new_file(owner)?;
                        break Ok(());
                    },
                    4 => {
                        update_existing_file()?;
                        break Ok(());
                    },
                    5 => {
                        move_file_to_trash()?;
                        break Ok(());
                    },
                    _ => unreachable!(),
                };
            },
            _ => println!("Please enter a valid number between 0 and 5."),
        }
    }
}


fn main() {
    let owner: (i64, String, String) = get_system_owner();
    println!("\t\tWelcome to your UniChain!\nusername: {}\ne-mail: {} ", owner.1, owner.2);
    let _ = system_menu(owner);
}


