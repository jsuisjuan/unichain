use std::io::{self, Write};
use log::info;

use unichain::commands::{list_files, view_file, store_file, update_file, delete_file};
use unichain::model::FileError;
use unichain::utils::get_system_owner;

pub fn run() -> Result<(), FileError> {
    let (_, username, email) = get_system_owner();
    println!("\n\t\tWelcome to your UniChain!\n\nusername: {}\ne-mail: {} ", username, email);
    loop {
        print_menu_options();
        match get_choosed_option()? {
            option => match option {
                0 => return Ok({
                    print!("\n");
                    info!("Exiting.\n")
                }),
                1 => list_files()?,
                2 => view_file()?,
                3 => store_file()?,
                4 => update_file()?,
                5 => delete_file()?,
                _ => unreachable!(),
            }
        }
    }
}

fn print_menu_options() {
    println!("\nWhat do you want to do?\n");
    println!("1. View list of stored files\n2. View a specific file\n3. Store a new file\n4. Update an existing file\n5. Move a file to trash\n0. Exit");
}

fn get_choosed_option() -> Result<u8, FileError> {
    loop {
        print!("\nChoose an option (0-5): ");
        io::stdout().flush().map_err(|e| FileError::IOError(e))?;
        let mut choosed_option = String::new();
        io::stdin().read_line(&mut choosed_option).map_err(|e| FileError::IOError(e))?;
        match choosed_option.trim().parse::<u8>() {
            Ok(num) if (0..=5).contains(&num) => return Ok(num),
            Ok(_) => println!("\n{}", FileError::InputError("The number must be between 0 and 5.".to_string())),
            Err(_) => println!("\n{}", FileError::InputError("Invalid digit found in string, please enter a number.".to_string())),
        }
    }
}


