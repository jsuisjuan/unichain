use std::io::{self, Write};

use unichain::{get_all_files, get_file, create_new_file, modify_file, delete_file};

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
    io::stdout().flush().expect("Failed to flush stdout");
    let mut choosed_option = String::new();
    io::stdin().read_line(&mut choosed_option).expect("Failed to read from stdin");
    match choosed_option.trim().parse::<u8>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Please enter a valid number between 0 and 5.".to_string())
    }
}

fn get_files() {
    get_all_files(PATH);
}

fn get_specific_file() {
    let file_id = 3;
    get_file(PATH, file_id);
}

fn store_new_file() {
    // let file_data;
    // create_new_file(file_data, PATH);
}

fn update_existing_file() {
    // let updated_file;
    let file_id = 3;
    // modify_file(PATH, file_id, updated_file);
}

fn move_file_to_trash() {
    let file_id = 3;
    delete_file(PATH, file_id);
}

fn system_menu() {
    print_menu_options();
    loop {
        match get_choosed_option() {
            Ok(option) if (0..=5).contains(&option) => {
                match option {
                    0 => {println!("Exiting..."); break;},
                    1 => get_files(),
                    2 => get_specific_file(),
                    3 => store_new_file(),
                    4 => update_existing_file(),
                    5 => move_file_to_trash(),
                    _ => unreachable!()
                }
            },
            _ => println!("Please enter a valid number between 0 and 5.")
        }
    }
}

fn main() {
    let owner: (i64, String, String) = get_system_owner();
    println!("\t\tWelcome to your UniChain!\nusername: {}\ne-mail: {} ", owner.1, owner.2);
    system_menu();
}


