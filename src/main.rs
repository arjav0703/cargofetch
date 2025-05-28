use std::process::Command;

fn main() {
    // println!("Welcome to rust fetch!");
    // let cargo_version = get_cargo_version();
    // println!("You are using Cargo version: {}", cargo_version.trim());
    get_cargo_version();
}

fn get_cargo_version() {
    let output = Command::new("cargo")
        .arg("--version")
        .spawn()
        .expect("Failed to execute command");
    let res = output.wait_with_output().expect("Failed to wait on child");
    // println!("Cargo version: {}", String::from_utf8_lossy(&res.stdout));
    // if output.status.success() {
        // String::from_utf8_lossy(&output.stdout).to_string()
    // } else {
        // panic!(
            // "Command failed: {}",
            // String::from_utf8_lossy(&output.stdout)
        // );
    // }
}
