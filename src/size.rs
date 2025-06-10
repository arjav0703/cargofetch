use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;

fn count_lines_in_file(file_path: &Path) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let line_count = reader.lines().count();
    Ok(line_count)
}

pub fn get_lines() -> usize {
    let folder_path = "./src";
    let mut total_lines = 0;

    for entry in WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
        if let Some(extension) = entry.path().extension().and_then(|s| s.to_str()) {
            if extension == "rs" {
                match count_lines_in_file(entry.path()) {
                    Ok(lines) => total_lines += lines,
                    Err(e) => eprintln!("Error reading file {}: {}", entry.path().display(), e),
                }
            }
        }
    }
    total_lines
}
