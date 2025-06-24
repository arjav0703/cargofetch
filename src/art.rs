use crate::cli::art_status;
use crate::cli::ArtType;
use crate::config::load_config;
use crate::size;
use crate::structure::Package;
use owo_colors::colors::CustomColor;
use owo_colors::OwoColorize;

/// Handler for displaying package information in ASCII art format.
pub fn handler(package: &Package, cargo_version: &str) {
    let lines = size::get_lines();

    let config = load_config().expect("Internal Error: Failed to load config");
    let enable_art = config.ascii_art;
    let art_type = config.art_type;

    let info = format_package_info(package, cargo_version, lines);
    print_art(&info, enable_art, art_type);
}

/// Formats the package information into a vector of strings for display.
fn format_package_info(package: &Package, cargo_version: &str, lines: usize) -> Vec<String> {
    let fields = [
        ("Cargo Version:", cargo_version),
        ("Package:", package.name.as_str()),
        ("Version:", package.version.as_str()),
        (
            "Description:",
            package.description.as_deref().unwrap_or("none"),
        ),
        ("Authors:", &package.authors.join(", ")),
        ("Dependencies:", &package.dependencies.len().to_string()),
        ("Lines of Code:", &lines.to_string()),
        ("Repo:", package.repository.as_deref().unwrap_or("none")),
        (
            "Documentation:",
            package.documentation.as_deref().unwrap_or("none"),
        ),
        ("License:", package.license.as_deref().unwrap_or("none")),
        ("Edition:", package.edition.as_str()),
    ];

    fields
        .iter()
        .map(|(label, value)| format!("{} {:>1}", label.fg::<CustomColor<247, 76, 0>>(), value))
        .collect()
}

// Prints the ASCII art and package information side by side.
fn print_art(info: &[String], enable_art: bool, art_type: String) {
    //let color = Color::Rgb(247, 76, 0);
    if !enable_art {
        for line in info {
            println!("{}", line);
        }
        return;
    }

    let ascii_art = art_gen(&art_type);
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

/// Returns the ascii_art as a string.
fn art_gen(art_type: &str) -> String {
    if art_status() {
        return String::new();
    }

    match art_type {
        "crab" => crab_art(),
        "rust" => rust_art(),
        _ => {
            eprintln!("Unknown art type: {}. Defaulting to crab art.", art_type);
            crab_art()
        }
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
