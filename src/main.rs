use std::process;

mod cli;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}


