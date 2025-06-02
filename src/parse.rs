use rust_fetch::art;
use rust_fetch::CargoMetadata;
use std::process::Command;

pub fn get_meta() {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .expect("Failed to execute command");

    jsonparser(output_to_string(output));
}

pub fn get_cargo_version() -> String {
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

fn jsonparser(json_data: String) {
    let metadata: CargoMetadata = serde_json::from_str(&json_data).expect("Failed to parse JSON");
    for package in &metadata.packages {
        art::output(package);
    }
}
