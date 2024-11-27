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



#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
 
    static INIT: Once = Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Info).init();
        });
    }

    mod cli {
        use crate::FileError;
        pub fn run() -> Result<(), FileError> {
            Ok(())
        }
    }

    #[test]
    fn test_run_app_success() {
        init_logger();
        let result = run_app();
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_app_error() {
        init_logger();
        mod cli {
            use crate::FileError;
            pub fn run() -> Result<(), FileError> {
                Err(FileError::IOError(std::io::Error::new(std::io::ErrorKind::NotFound, "mock error")))
            }
        }
        let result = run_app();
        assert!(result.is_err());
    }
}

