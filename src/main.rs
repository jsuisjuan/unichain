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
    use std::io::Write;
    use std::sync::Once;
    use log::{info, error};
    use crate::{FileError, run_app}; // Adjust the import as per your project structure

    // Initialize logger only once for the tests to avoid duplicate initialization errors
    static INIT: Once = Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Info)
                .init();
        });
    }

    // Mock implementation for cli::run that simulates different behaviors
    mod cli {
        use crate::FileError;

        pub fn run() -> Result<(), FileError> {
            // Simulate a successful run (can be changed to return an error in other tests)
            Ok(())
        }
    }

    #[test]
    fn test_run_app_success() {
        init_logger(); // Initialize logger before running the test
        
        // Simulate a successful run
        let result = run_app();
        
        // Assert that the result is Ok, indicating success
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_app_error() {
        init_logger(); // Initialize logger before running the test

        // Modify the mock to simulate an error case
        mod cli {
            use crate::FileError;

            pub fn run() -> Result<(), FileError> {
                // Simulate an error (returning a mock error for testing)
                Err(FileError::IOError(std::io::Error::new(std::io::ErrorKind::NotFound, "mock error")))
            }
        }

        // Capture logs to verify correct logging
        let result = run_app();

        // Assert that the result is an error
        assert!(result.is_err());
    }
}

