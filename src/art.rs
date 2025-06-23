use crate::cli::art_status;
use crate::cli::art_type;
use crate::cli::ArtType;
use crate::size;
use crate::structure::Package;
use owo_colors::colors::CustomColor;
use owo_colors::OwoColorize;

/// Handler for displaying package information in ASCII art format.
pub fn handler(package: &Package, cargo_version: &String) {
    let lines = size::get_lines();

    let info = format_package_info(package, cargo_version, lines);
    print_art(&info);
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
fn print_art(info: &[String]) {
    //let color = Color::Rgb(247, 76, 0);
    if art_status() {
        for line in info {
            println!("{}", line);
        }
    } else {
        let ascii_art = art_gen();
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
}

/// Returns the ascii_art as a string.
fn art_gen() -> String {
    if art_status() {
        return String::new();
    }

    match art_type() {
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
   `mMMMm                `-:o+:/mMMMm`
   -ssNMMMyomo            smohMMMNss-
     `NNNMs+mN/-`      `-/Nd/yMNNN`
      ` -MMNMMMMMNmmmmNMMMMMNMM- `
         -``MNsNMMNMMNMMNsNM``-
               `  :y.`yy`.y:  `
    "#
    .to_string()
}
