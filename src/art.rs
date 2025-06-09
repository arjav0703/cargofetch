use crate::cli::art_status;
use crate::Package;
use owo_colors::OwoColorize;

pub fn handler(package: &Package, cargo_version: &String) {
    let info = format_package_info(package, cargo_version);
    print_art(&info);
}

fn format_package_info(package: &Package, cargo_version: &String) -> Vec<String> {
    let fields = [
        ("Cargo Version:", cargo_version.as_str()),
        ("Package Name:", package.name.as_str()),
        ("Version:", package.version.as_str()),
        ("Dependencies:", &package.dependencies.len().to_string()),
        (
            "Repository:",
            package.repository.as_deref().unwrap_or("null"),
        ),
    ];

    fields
        .iter()
        .map(|(label, value)| format!("{} {}", label.red(), value))
        .collect()
}

fn print_art(info: &[String]) {
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
            println!("{:<40}  {}", art_line.red().bold(), side_text);
        }
    }
}

fn art_gen() -> String {
    if art_status() {
        return String::new();
    }

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
