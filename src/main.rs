use std::{io::{self, Write}, path::Path};

use unichain::{create_new_file, delete_file, get_all_files, get_file, modify_file, model::{File, FileData}};

const PATH: &str = "../assets/";

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
    let mut choosed_option: String = String::new();
    io::stdin().read_line(&mut choosed_option).map_err(|_| "Failed to read from stdin")?;
    match choosed_option.trim().parse::<u8>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Please enter a valid number between 0 and 5.".to_string())
    }
}

fn get_files() -> Result<(), String> {
    println!("All the files:");
    let files: Vec<File> = get_all_files(PATH)?;
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
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).map_err(|e| e.to_string())?;
    let file_path = file_path.trim();
    let path = Path::new(file_path);
    let filename = match path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => {
            eprintln!("Invalid file path");
            return Err("Invalid file path".to_string());
        }
    };
    print!("Your current file name is: {}. Do you want to change it? (Y/N): ", filename);
    io::stdout().flush().map_err(|e| e.to_string())?;
    let mut change_name_response = String::new();
    io::stdin().read_line(&mut change_name_response).map_err(|e| e.to_string())?;
    let change_name_response = change_name_response.trim().to_lowercase();
    let final_name = if change_name_response == "y" {
        print!("Enter the new file name: ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        let mut new_name = String::new();
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
    let _file: File = get_file(PATH, file_id)?;
    // this function will be very complex
    // modify_file(PATH, file_id, updated_file);
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
                    1 => {get_files()?; break Ok(());},
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


