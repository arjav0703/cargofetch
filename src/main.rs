use anyhow::Result;
use cargofetch::checks;
use cargofetch::parse;
use cargofetch::render;

fn main() -> Result<()> {
    checks::env_check()?;

    let metadata = parse::get_metadata()?;
    let cargo_version = parse::get_cargo_version()?;
    for package in &metadata.packages {
        render::handler(package, &cargo_version)?;
    }

    Ok(())
}
