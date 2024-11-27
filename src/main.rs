use std::process;
use log::{info, error};
use env_logger;

use unichain::model::FileError;

mod cli;

fn main() {
    if let Err(_) = run_app() {
        process::exit(1);
    }
}

fn run_app() -> Result<(), FileError> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    info!("Initializing the program.");
    if let Err(e) = cli::run() {
        error!("Application error: {e}");
        return Err(e);
    };
    Ok(())
}
