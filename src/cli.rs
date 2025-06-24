use clap::{Parser, ValueEnum};

#[derive(Debug, ValueEnum, Clone)]
pub enum ArtType {
    Crab,
    Rust,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub disable_art: bool,

    #[arg(long, value_enum)]
    pub art_type: Option<ArtType>,
}
/// Returns false if art is allowed (default_value = false)
pub fn art_status() -> bool {
    let args = Cli::parse();

    args.disable_art
}

/// Returns the type of art to be displayed ('crab' or 'rust')
pub fn art_type() -> String {
    let args = Cli::parse();
    match args.art_type.unwrap_or(ArtType::Crab) {
        ArtType::Crab => "crab".to_string(),
        ArtType::Rust => "rust".to_string(),
    }
}
pub fn init() {
    let _ = Cli::parse();
}
