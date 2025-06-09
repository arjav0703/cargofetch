pub mod parse;
use size::get_lines;
use which::which;
mod size;
fn main() {
    if which("cargo").is_err() {
        panic!("Application error: Cargo not found. Please install cargo and make sure it is in your PATH.");
    }
    let l = get_lines();
    println!("Total lines of code in src: {}", l);

    parse::init();
}
