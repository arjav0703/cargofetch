use std::process::Command;

fn main() {
    println!("Welcome to rust fetch!");

    let x = Command::new("cargo").arg("--version").output().expect("Cargo not found");
    let _ver = x.stdout;
    println!("Cargo version: {}", String::from_utf8_lossy(&_ver));
}
