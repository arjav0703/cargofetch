use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(long, default_value = "false")]
    no_ascii_art: bool,
}

pub fn art_status() -> bool {
    let args = Cli::parse();

    args.no_ascii_art
}
