pub mod parse;
use std::fs;
use which::which;

fn main() {
    env_test();

    parse::init();
}

fn env_test() {
    if which("cargo").is_err() {
        panic!("Application error: Cargo not found. Please install cargo and make sure it is in your PATH.");
    }
    match fs::read("Cargo.toml") {
        Ok(..) => {}
        Err(..) => {
            panic!("Rust project not found. Cargo.toml is missing");
        }
    }
}
