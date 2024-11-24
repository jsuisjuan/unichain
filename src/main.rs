use std::process;
use log::{info, error};
use env_logger;

mod cli;

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    info!("Initializing the program.");
    if let Err(e) = cli::run() {
        error!("Application error: {e}");
        process::exit(1);
    };
}


