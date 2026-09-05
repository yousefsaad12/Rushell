use crate::commands::{ cd, echo, exit, pwd, type_cmd };
use crate::external::{ executor, finder };

pub struct CommandOutput {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

pub fn execute_builtin(command: &str, args: &[&str]) -> Option<CommandOutput> {
    match command {
        "echo" =>
            Some(CommandOutput {
                stdout: Some(echo::run(args)),
                stderr: None,
            }),
        "exit" => {
            exit::run();
        }
        "type" =>
            Some(CommandOutput {
                stdout: Some(type_cmd::run(args.first().copied().unwrap_or(""))),
                stderr: None,
            }),
        "pwd" =>
            Some(CommandOutput {
                stdout: Some(pwd::run()),
                stderr: None,
            }),
        "cd" => {
            cd::run(args);
            Some(CommandOutput {
                stdout: None,
                stderr: None,
            })
        }
        "" =>
            Some(CommandOutput {
                stdout: None,
                stderr: None,
            }),
        _ => None,
    }
}

pub fn execute_external(
    command: &str,
    args: &[&str],
    output_redirection: Option<&str>,
    output_append: bool,
    error_redirection: Option<&str>,
    error_append: bool
) -> CommandOutput {
    match finder::find_exe(command) {
        Some(path) => {
            if
                let Err(error) = executor::run(
                    &path,
                    command,
                    args,
                    output_redirection,
                    output_append,
                    error_redirection,
                    error_append
                )
            {
                return CommandOutput {
                    stdout: None,
                    stderr: Some(format!("{}: {}", command, error)),
                };
            }

            CommandOutput {
                stdout: None,
                stderr: None,
            }
        }
        None =>
            CommandOutput {
                stdout: None,
                stderr: Some(format!("{}: command not found", command)),
            },
    }
}
