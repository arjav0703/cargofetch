use std::process::Command;

pub fn get_meta() -> String {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .expect("Failed to execute command");
    
    // todo: create a fn for this success check as it is repeated
    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        panic!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

pub fn get_cargo_version() -> String {
    let output = Command::new("cargo")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        panic!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}
