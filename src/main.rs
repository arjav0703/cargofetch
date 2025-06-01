use rust_fetch::{self, get_cargo_version, get_meta};

fn main() {
    let meta = get_meta();
    let cargo_version = get_cargo_version();
    println!("You are using Cargo version: {}", cargo_version.trim());
    println!("Cargo metadata: {}", meta.trim());
}
