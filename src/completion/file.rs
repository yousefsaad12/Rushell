use rustyline::completion::Pair;
use std::path::PathBuf;

pub fn complete(prefix: &str) -> (usize, Vec<Pair>) {
    let (directory_prefix, file_prefix) = match prefix.rfind('/') {
        Some(index) => (&prefix[..=index], &prefix[index + 1..]),
        None => ("", prefix),
    };
    let directory = if directory_prefix.is_empty() {
        PathBuf::from(".")
    } else {
        PathBuf::from(directory_prefix)
    };

    let Ok(entries) = std::fs::read_dir(&directory) else {
        return (prefix.len(), Vec::new());
    };

    let mut candidates: Vec<Pair> = entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_str()?.to_string();
            if !name.starts_with(file_prefix) {
                return None;
            }

            let candidate = format!("{directory_prefix}{name}");
            let replacement = if entry.path().is_dir() {
                format!("{name}/")
            } else {
                format!("{name} ")
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
