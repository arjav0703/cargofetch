//use crate::cli::art_status;
//use crate::cli::ArtType;
use crate::cli::Cli;
use crate::config::load_config;
use crate::config::save_config;
use crate::size;
use crate::structure::ArtType;
use crate::structure::Package;
use anyhow::Result;
use clap::Parser;
use owo_colors::OwoColorize;
use owo_colors::colors::CustomColor;

/// Handler for displaying package information in ASCII art format.
pub fn handler(package: &Package, cargo_version: &str) -> Result<()> {
    let cli = Cli::parse();

    let mut cfg = load_config()?;

    cfg.enable_art = !cli.disable_art;

    save_config(&cfg)?;

    let lines = size::get_lines();
    let info = format_package_info(PackageInfo {
        cargo_version,
        package,
        lines,
        git_commits: get_commits()?,
    });

    print_art(&info, cfg.enable_art, cfg.art_type);

    Ok(())
}

/// Gets the number of commits in the repository
fn get_commits() -> Result<usize> {
    use std::process::Command;

    let output = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .output()?;

    if output.status.success() {
        let count_str = String::from_utf8_lossy(&output.stdout);
        let count = count_str.trim().parse::<usize>()?;
        Ok(count)
    } else {
        Err(anyhow::anyhow!(
            "Failed to get commit count: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

struct PackageInfo<'a> {
    cargo_version: &'a str,
    package: &'a Package,
    lines: usize,
    git_commits: usize,
}

/// Formats the package information into a vector of strings for display.
fn format_package_info(info: PackageInfo) -> Vec<String> {
    let fields = [
        ("Cargo Version:", info.cargo_version),
        ("Package:", info.package.name.as_str()),
        ("Version:", info.package.version.as_str()),
        (
            "Description:",
            info.package.description.as_deref().unwrap_or("none"),
        ),
        ("Authors:", &info.package.authors.join(", ")),
        (
            "Dependencies:",
            &info.package.dependencies.len().to_string(),
        ),
        ("Lines of Code:", &info.lines.to_string()),
        (
            "Repo:",
            info.package.repository.as_deref().unwrap_or("none"),
        ),
        (
            "Documentation:",
            info.package.documentation.as_deref().unwrap_or("none"),
        ),
        (
            "License:",
            info.package.license.as_deref().unwrap_or("none"),
        ),
        ("Edition:", info.package.edition.as_str()),
        ("Git Commits:", &info.git_commits.to_string()),
    ];

    fields
        .iter()
        .map(|(label, value)| format!("{} {:>1}", label.fg::<CustomColor<247, 76, 0>>(), value))
        .collect()
}

// Prints the ASCII art and package information side by side.
fn print_art(info: &[String], enable_art: bool, art_type: ArtType) {
    //dbg!(enable_art);
    //let color = Color::Rgb(247, 76, 0);
    if !enable_art {
        for line in info {
            println!("{line}");
        }
        return;
    }

    let ascii_art = art_gen(art_type , enable_art);
    let ascii_lines: Vec<&str> = ascii_art.trim_matches('\n').lines().collect();

    for (art_line, side_text) in ascii_lines
        .iter()
        .zip(info.iter().chain(std::iter::repeat(&"".to_string())))
    {
        println!(
            "{:<40}  {}",
            art_line.fg::<CustomColor<247, 76, 0>>(),
            side_text
        );
    }
}

/// Returns the ascii_art as a string based on the art_type provided.
fn art_gen(art_type: ArtType, enable_art: bool) -> String {
    //dbg!(art_type);
    if !enable_art {
        return String::new();
    }
    match art_type {
        ArtType::Crab => crab_art(),
        ArtType::Rust => rust_art(),
    }
}

fn crab_art() -> String {
    r#"
                 R RR RR   
              R RRRRRRRR R          R
 R RR       R RRRRRRRRRRRRR R      RR
rR RRR    R RRRRRRRRRRRRRRRRR R   RRR R
RRR RR   RRRRRRRRRRRRRRRRRRRRRRR  RRRRR
 RRRRR  RRRRRRRRRRRRRRRRRRRRRRRR  RRRR
  RRR RRRRRRRRRRRRRRRRRRRRRRRRRRRR RR
    R  RRRRRRRRRR=  RR = RRRRRRRRRRR
     RRRRRRRRRRRR=  RR = RRRRRRRRRR
      RRRRRRRRRRR   RR   RRRRRRRRRR
     RR==RRRRRRRRRRRRRRRRRRRRRR===RR
     RR =  ==RRRRRRR  RRRRRR==  = RR
      RR =     ===========     = RR
       RR                        R
        R                       R
         R              
    "#
    .to_string()
}

fn rust_art() -> String {
    r#"
    
             `  :y.`yy`.y:  `
         -``MNsNMMNNNNMMNsNM``-
      ` -MMNMMMMNNm``NNNMMMMNMM- `
     `NNNMMMdo:` `+md/  `:odMMMNNN`
   -ssNMMNo.                .oNMMNss-
   `mMMMMNmmmmmmmmmmmmmmmdy+` `sMMMm`
 `mMMMMMMMMMMMMMMMMMMMMMMMMMN/  hMMMMm`
 -oMN-:Ny:mMMMMMm    oNMMMMMm  oN::MMo-
.yMMMhhh+ dMMMMMd:::::+mMMMMN/ odyhMMMy.
-sNMMy    dMMMMMMMMMMMMMMMMs`    `yMMNs-
-sNMMy    dMMMMMNyyyydMMMMMMy   .odMMNs-
.yMMMm   dMMMMMh     +MMMMMM+   sMMMMMy.
 -oMMMMMMMMMMMMMMMMM+  mMMMMMMMMMMMMMo-
 `mMMMMMMMMMMMMMMMMM+  :NMMMMMMMMMMMMm`
  `mMMMm                 `-:o+:/mMMMm`
   -ssNMMMyomo            smohMMMNss-
     `NNNMs+mN/-`      `-/Nd/yMNNN`
      ` -MMNMMMMMNmmmmNMMMMMNMM- `
         -``MNsNMMNMMNMMNsNM``-
            `  :y.`yy`.y:  `
    "#
    .to_string()
}
