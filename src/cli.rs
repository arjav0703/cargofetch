use crate::structure::ArtType;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub disable_art: bool,

    #[arg(long, value_enum)]
    pub art_type: Option<ArtType>,

    #[arg(long, short)]
    pub update: bool,
}

pub struct Terminal;

impl Terminal {
    /// Returns false if art is allowed (default_value = false)
    pub fn art_status() -> bool {
        let args = Cli::parse();

        args.disable_art
    }

    pub fn update_status() -> bool {
        let args = Cli::parse();

        args.update
    }

    pub fn art_type() -> ArtType {
        let args = Cli::parse();
        args.art_type.unwrap_or(ArtType::Crab)
    }
    pub fn init() {
        let _ = Cli::parse();
    }
}
