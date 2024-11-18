use std::io;

fn get_system_owner() -> (i64, String, String) {
    (
        2454826096558341,
        String::from("Juan Carvalho Silva de Lima"),
        String::from("juanc.s.delima@gmail.com"),
    )
}

fn system_menu() {
    println!("What do you want to do?");
    println!("
        1. View list of stored files\n
        2. View a specific file\n
        3. Store a new file\n
        4. Update an existing file\n
        5. Move a file to trash\n
        0. Exit
    ");
    let choice: u8 = loop {
        let mut choosed_option: String = String::new();
        io::stdin().read_line(&mut choosed_option).expect("Failed to read from stdin");
        match choosed_option.trim().parse::<u8>() {
            Ok(option) if (0..=5).contains(&option) => break option,
            _ => println!("Please enter a valid number between 0 and 5.")
        }
    };
    match choice {
        0 => println!("Exiting..."),
        1 => println!("Viewing list of stored files..."),
        2 => println!("Viewing a specific file..."),
        3 => println!("Storing a new file..."),
        4 => println!("Updating an existing file..."),
        5 => println!("Moving a file to trash..."),
        _ => unreachable!()
    };
}

fn main() {
    let owner: (i64, String, String) = get_system_owner();
    println!("\t\tWelcome to your UniChain!\nusername: {}\ne-mail: {} ", owner.1, owner.2);
    system_menu();
}


