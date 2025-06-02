use crate::Package;
use owo_colors::OwoColorize;

pub fn output(package: &Package) {
    let info = [
        format!("Package Name:{}", package.name),
        format!("Version: {}", package.version),
        format!(
            "Reposiory: {}",
            package.repository.as_deref().unwrap_or("null")
        ),
    ];

    let _dependencies = package
        .dependencies
        .iter()
        .map(|d| d.name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    println!("info: {:?}", info);
    print_art();
}

fn print_art() {
    let ascii_art = r#"
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
    println!("{}", ascii_art.red().bold());
}
