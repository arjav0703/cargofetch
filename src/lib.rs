pub mod checks;
pub mod cli;
pub mod config;
pub mod parse;
pub mod render;
pub mod size;
pub mod structure;

use render::{format_package_info, format_with_art};

pub use crate::render::PackageInfo;
pub use crate::structure::ArtType;

/// Build a [`PackageInfo`] and format it, returning the display lines.
///
/// This is the low-level entry point: callers that have already built a
/// `PackageInfo` can pass it directly to get the formatted strings.
pub fn generate(pkg_info: PackageInfo) -> Vec<String> {
    format_package_info(pkg_info)
}

/// Run the full cargofetch pipeline for the current directory and return
/// the formatted display lines.
///
/// Calls `cargo metadata`, `cargo --version`, counts source lines/size, and
/// counts git commits — then formats everything into a `Vec<String>` using
/// the same logic as the CLI.
///
/// Returns an error if `cargo metadata` or `cargo --version` fail.
pub fn fetch() -> anyhow::Result<Vec<String>> {
    let metadata = parse::get_metadata()?;
    let cargo_version = parse::get_cargo_version()?;

    let package = metadata
        .packages
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No packages found in cargo metadata output"))?;

    let size = size::get_size();

    let git_commits = count_git_commits().unwrap_or(0);

    Ok(format_with_art(
        PackageInfo {
            cargo_version: &cargo_version,
            package: &package,
            size,
            git_commits,
        },
        ArtType::Crab,
    ))
}

fn count_git_commits() -> anyhow::Result<usize> {
    use std::process::Command;
    let output = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .output()?;
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        Ok(s.trim().parse::<usize>()?)
    } else {
        Err(anyhow::anyhow!("git rev-list failed"))
    }
}
