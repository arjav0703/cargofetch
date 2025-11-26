use anyhow::Result;

mod checks;
pub mod cli;
pub mod config;
pub mod parse;
pub mod render;
pub mod size;
pub mod structure;

fn main() -> Result<()> {
    checks::env_check()?;

    let metadata = parse::get_metadata()?;
    let cargo_version = parse::get_cargo_version()?;
    for package in &metadata.packages {
        render::handler(package, &cargo_version)?;
    }

    Ok(())
}
