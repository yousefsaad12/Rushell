use std::collections::BTreeSet;
use std::env;
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn is_executable(metadata: &Metadata) -> bool {
    metadata.is_file() && (metadata.permissions().mode() & 0o111) != 0
}

pub fn find_exe(cmd: &str) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;

    for dir in env::split_paths(&path_var) {
        let candidate = dir.join(cmd);
        if let Ok(metadata) = std::fs::metadata(&candidate) {
            if is_executable(&metadata) {
                return Some(candidate);
            }
        }
    }
    None
}

pub fn find_executables(prefix: &str) -> Vec<String> {
    let Some(path_var) = env::var_os("PATH") else {
        return Vec::new();
    };

    let mut executables = BTreeSet::new();

    for dir in env::split_paths(&path_var) {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };

        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let Some(name) = file_name.to_str() else {
                continue;
            };
            if !name.starts_with(prefix) {
                continue;
            }

            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            if is_executable(&metadata) {
                executables.insert(name.to_string());
            }
        }
    }

    executables.into_iter().collect()
}
