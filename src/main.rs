mod cli;
pub mod parse;
use which::which;

fn main() {
    if which("cargo").is_err() {
        panic!("Application error: Cargo not found. Please install cargo and make sure it is in your PATH.");
    }

    parse::init();
}
