use anyhow::Result;
use std::fs;
use which::which;

/// Checks the environment for necessary tools and files.
pub fn env_check() -> Result<()> {
    rust_check()?;
    project_check()?;
    Ok(())
}

/// Checks if Cargo is installed and available in the PATH.
fn rust_check() -> Result<()> {
    if which("cargo").is_err() {
        Err(anyhow::anyhow!(
            "Application error: Cargo not found. Please install cargo and make sure it is in your PATH.",
        ))
    } else {
        Ok(())
    }
}

/// Checks if the current directory contains a Cargo project by looking for Cargo.toml.
fn project_check() -> Result<()> {
    match fs::read("Cargo.toml") {
        Ok(..) => Ok(()),
        Err(..) => Err(anyhow::anyhow!(
            "Cargo.toml not found in the current directory. Please run this command from the root of your Cargo project."
        )),
    }
}
