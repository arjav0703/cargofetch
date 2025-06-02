use rust_fetch::art;
use rust_fetch::CargoMetadata;
use std::process::Command;

pub fn init() {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .expect("Failed to execute command");

    send_to_art(output_to_string(output));
}

fn get_cargo_version() -> String {
    let output = Command::new("cargo")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    // this will return a String

    output_to_string(output)
        .split_whitespace()
        .nth(1)
        .unwrap_or("")
        .to_string()
}

fn output_to_string(output: std::process::Output) -> String {
    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        panic!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn send_to_art(json_data: String) {
    let cargo_version = get_cargo_version();

    let metadata: CargoMetadata = serde_json::from_str(&json_data).expect("Failed to parse JSON");
    for package in &metadata.packages {
        art::handler(package, &cargo_version);
    }
}
