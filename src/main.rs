use rust_fetch::{self, get_meta, get_cargo_version};

fn main() {
    let meta = get_meta();
    let cargo_version = get_cargo_version();

    println!("You are using Cargo version: {}", cargo_version.trim());
    println!("Cargo metadata: {}", meta.trim());
}

