use crate::Package;
use owo_colors::OwoColorize;

pub fn handler(package: &Package, cargo_version: &String) {
    let dependencies = package.dependencies.len();

    let info = [
        format!("Cargo Version: {}", cargo_version),
        format!("Package Name:{}", package.name),
        format!("Version: {}", package.version),
        format!("Dependencies: {} (Cargo)", dependencies),
        format!(
            "Repository: {}",
            package.repository.as_deref().unwrap_or("null")
        ),
    ];

    print_art(info);
}

fn print_art(info: [String; 5]) {
    let ascii_art = r#"
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
    "#;

    let ascii_lines: Vec<&str> = ascii_art.trim_matches('\n').lines().collect();
    let err = String::from("");

    // Print each line, aligning ASCII with side text
    for i in 0..ascii_lines.len().max(info.len()) {
        let art_line = ascii_lines.get(i).unwrap_or(&"");
        let side_text = info.get(i).unwrap_or(&err);
        println!("{:<40}  {}", art_line.red().bold(), side_text);
    }

    //println!("{}", ascii_art.red().bold());
}
