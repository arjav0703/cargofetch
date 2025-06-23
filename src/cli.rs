use clap::{Parser, ValueEnum};

#[derive(Debug, ValueEnum, Clone)]
pub enum ArtType {
    Crab,
    Rust,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    no_ascii_art: bool,

    #[arg(long, value_enum, default_value_t = ArtType::Crab)]
    art_type: ArtType,
}
/// Returns false if art is allowed (default_value = false)
pub fn art_status() -> bool {
    let args = Cli::parse();

    args.no_ascii_art
}

pub fn art_type() -> ArtType {
    let args = Cli::parse();

    args.art_type
}
