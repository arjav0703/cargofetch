use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;

/// Counts the number of lines in a file.
fn count_lines_in_file(file_path: &Path) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let line_count = reader.lines().count();
    Ok(line_count)
}

pub struct Size {
    lines: usize,
    filesize: usize,
    number_of_files: usize,
}

impl Size {
    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn filesize(&self) -> usize {
        let filesize_kb = self.filesize as f64 / 1024.0;
        filesize_kb.ceil() as usize
    }
    pub fn number_of_files(&self) -> usize {
        self.number_of_files
    }
}

/// Returns the number of lines in the project (located at ./src)
pub fn get_size() -> Size {
    let folder_path = "./src";
    let mut total_lines = 0;
    let mut total_filesize = 0;
    let mut total_number_of_files = 0;

    for entry in WalkDir::new(folder_path).into_iter().filter_map(Result::ok) {
        if let Some(extension) = entry.path().extension().and_then(|s| s.to_str())
            && extension == "rs"
        {
            total_number_of_files += 1;
            match count_lines_in_file(entry.path()) {
                Ok(lines) => total_lines += lines,
                Err(e) => eprintln!("Error reading file {}: {}", entry.path().display(), e),
            }
            let size = std::fs::metadata(entry.path())
                .map(|m| m.len() as usize)
                .unwrap_or(0);
            total_filesize += size;
        }
    }
    Size {
        lines: total_lines,
        filesize: total_filesize,
        number_of_files: total_number_of_files,
    }
}
