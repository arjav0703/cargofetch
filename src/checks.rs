use std::fs;
use which::which;

/// Checks the environment for necessary tools and files.
pub fn env_check() {
    rust_check();
    project_check();
}

/// Checks if Cargo is installed and available in the PATH.
fn rust_check() {
    if which("cargo").is_err() {
        panic!("Application error: Cargo not found. Please install cargo and make sure it is in your PATH.");
    }
}

/// Checks if the current directory contains a Cargo project by looking for Cargo.toml.
fn project_check() {
    match fs::read("Cargo.toml") {
        Ok(..) => {}
        Err(..) => {
            panic!("Cargo.toml not found in the current directory. Please run this command from the root of your Cargo project.");
        }
    }
}
