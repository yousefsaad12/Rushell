use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub fn find_exe(cmd: &str) -> Option<PathBuf> {
    let path_var = env::var("PATH").ok()?;
    for dir in env::split_paths(&path_var) {
        let candidate = dir.join(cmd);
        if candidate.is_file() {
            if let Ok(metadata) = std::fs::metadata(&candidate) {
                if metadata.permissions().mode() & 0o111 != 0 {
                    return Some(candidate);
                }
            }
        }
    }
    None
}
