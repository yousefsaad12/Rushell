use rustyline::completion::Pair;
use std::path::{Path, PathBuf};

pub fn complete(prefix: &str) -> (usize, Vec<Pair>) {
    let path = Path::new(prefix);
    let directory = path
        .parent()
        .filter(|directory| !directory.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let file_prefix = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let display_directory = if directory == Path::new(".") {
        String::new()
    } else if directory == Path::new("/") {
        "/".to_string()
    } else {
        format!("{}/", directory.to_string_lossy())
    };
    let read_directory = if directory.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        directory.to_path_buf()
    };

    let Ok(entries) = std::fs::read_dir(read_directory) else {
        return (prefix.len(), Vec::new());
    };

    let mut candidates: Vec<Pair> = entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_str()?.to_string();
            if !name.starts_with(file_prefix) {
                return None;
            }

            let candidate_path = entry.path();
            let candidate = format!("{display_directory}{name}");
            let replacement = if candidate_path.is_dir() {
                format!("{candidate}/")
            } else {
                format!("{candidate} ")
            };
            Some(Pair {
                display: candidate,
                replacement,
            })
        })
        .collect();
    candidates.sort_by(|left, right| left.display.cmp(&right.display));

    let start = prefix.len() - file_prefix.len();
    (start, candidates)
}
