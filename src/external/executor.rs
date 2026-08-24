use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::redirect_io::create_redirect_file;

pub fn run(
    path: &Path,
    command: &str,
    args: &[&str],
    output_redirection: Option<&str>,
    output_append: bool,
    error_redirection: Option<&str>,
    error_append: bool,
) {
    let mut cmd = Command::new(path);

    cmd.arg0(command).args(args);

    match output_redirection {
        Some(file_path) => {
            let file = create_redirect_file(file_path, output_append)
                .expect("failed to open output file");

            cmd.stdout(Stdio::from(file));
        }

        None => {
            cmd.stdout(Stdio::inherit());
        }
    }

    match error_redirection {
        Some(file_path) => {
            let file = create_redirect_file(file_path, error_append)
                .expect("failed to open error file");

            cmd.stderr(Stdio::from(file));
        }

        None => {
            cmd.stderr(Stdio::inherit());
        }
    }

    cmd.status().expect("failed to execute command");
}