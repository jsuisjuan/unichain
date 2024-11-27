use std::io::{self, Write};
use log::{info, warn};

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
            Ok(_) => warn!("The number must be between 0 and 5."),
            Err(_) => warn!("Invalid digit found in string, please enter a number.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};
    use crate::{FileError, get_choosed_option}

    // Helper function to simulate user input
    fn mock_input(input: &str) -> io::Result<String> {
        let mut input = input.to_string();
        let mut stdout = io::stdout();
        let mut input_buf = input.as_bytes().to_vec();
        stdout.write_all(&input_buf)?;
        Ok(input)
    }

    #[test]
    fn test_valid_option_0() {
        let input = "0\n";
        mock_input(input).expect("Failed to simulate input");

        let result = get_choosed_option();
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_valid_option_5() {
        let input = "5\n";
        mock_input(input).expect("Failed to simulate input");

        let result = get_choosed_option();
        assert_eq!(result, Ok(5));
    }

    #[test]
    fn test_invalid_option_above_range() {
        let input = "6\n";
        mock_input(input).expect("Failed to simulate input");

        let result = get_choosed_option();
        // Ensure the function warns about the invalid input
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_option_non_numeric() {
        let input = "abc\n";
        mock_input(input).expect("Failed to simulate input");

        let result = get_choosed_option();
        // Ensure the function warns about the invalid input
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_input() {
        let input = "\n";
        mock_input(input).expect("Failed to simulate input");

        let result = get_choosed_option();
        assert!(result.is_err());
    }
}

