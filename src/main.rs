pub mod parse;
use which::which;

fn main() {
    if which("cargo").is_err() {
        panic!("Application error: Cargo not found. Please install cargo and make sure it is in your PATH.");
    }

    parse::get_meta();
    let cargo_version = parse::get_cargo_version();
    println!("You are using Cargo version: {}", cargo_version.trim());
}
