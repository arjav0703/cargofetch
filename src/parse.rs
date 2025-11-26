use crate::structure::CargoMetadata;
use anyhow::Result;
use std::process::Command;

trait OutputExt {
    fn into_string(self) -> Result<String>;
}

impl OutputExt for std::process::Output {
    fn into_string(self) -> Result<String> {
        if self.status.success() {
            Ok(String::from_utf8_lossy(&self.stdout).to_string())
        } else {
            Err(anyhow::anyhow!(
                "Command failed with error: {}",
                String::from_utf8_lossy(&self.stderr)
            ))
        }
    }
}

/// Parses the output of cargo metadata and returns the CargoMetadata struct.
pub fn get_metadata() -> Result<CargoMetadata> {
    let metadata_json = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()?
        .into_string()?;

    Ok(serde_json::from_str(&metadata_json)?)
}

/// Retrieves the Cargo version from the command line.
pub fn get_cargo_version() -> Result<String> {
    let output = Command::new("cargo").arg("--version").output()?;

    Ok(output
        .into_string()?
        .split_whitespace()
        .nth(1)
        .unwrap_or("")
        .to_string())
}
