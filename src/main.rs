pub mod parse;

fn main() {
    parse::get_meta();
    let cargo_version = parse::get_cargo_version();
    println!("You are using Cargo version: {}", cargo_version.trim());
}
