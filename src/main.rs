use std::process::Command;

fn main() {
    println!("Welcome to rust fetch!");
    let cargo_version = get_cargo_version();
    println!("You are using Cargo version: {}", cargo_version.trim());
}

fn get_cargo_version() {
    let output = Command::new("cargo")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string();
    } else {
        panic!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}
