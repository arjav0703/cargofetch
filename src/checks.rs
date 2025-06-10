use std::fs;
use which::which;

pub fn env_check() {
    rust_check();
    project_check();
}

fn rust_check() {
    if which("cargo").is_err() {
        panic!("Application error: Cargo not found. Please install cargo and make sure it is in your PATH.");
    }
}

fn project_check() {
    match fs::read("Cargo.toml") {
        Ok(..) => {}
        Err(..) => {
            panic!("Cargo.toml not found in the current directory. Please run this command from the root of your Cargo project.");
        }
    }
}
