use std::io::{self, Write};

pub mod commands;
use commands::{list_files, view_file, store_file, update_file, delete_file};

pub mod utils;
use utils::{get_default_file, process_modified_file, update_accessed_file_date};

pub fn run() -> Result<(), FileError> {
    let owner: (i64, String, String) = get_system_owner();
    println!("\t\tWelcome to your UniChain!\nusername: {}\ne-mail: {} ", owner.1, owner.2);
    print_menu_options();
    loop {
        match get_choosed_option()? {
            option if (0..=5).contains(&option) => {
                match option {
                    0 => {
                        println!("Exiting...");
                        break Ok(());
                    },
                    1 => list_files()?,
                    2 => view_file()?,
                    3 => store_file()?,
                    4 => update_file()?,
                    5 => move_file_to_trash()?,
                    _ => unreachable!(),
                };
            },
            _ => {
                println!("Please enter a valid number between 0 and 5.");
                continue;
            }
        }
    }
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


