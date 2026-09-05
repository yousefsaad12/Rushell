use crate::execution::command::CommandOutput;
use crate::parser::redirection::ParsedCommand;
use crate::redirection::create_redirect_file;
use std::io::Write;

pub fn handle(output: CommandOutput, parsed_command: &ParsedCommand) {
    if let Some(text) = output.stdout {
        match parsed_command.output_redirection {
            Some(path) =>
                match create_redirect_file(path, parsed_command.output_append) {
                    Ok(mut file) => {
                        if let Err(error) = writeln!(file, "{}", text) {
                            eprintln!("failed to write output to {}: {}", path, error);
                        }
                    }
                    Err(error) => eprintln!("failed to open output file {}: {}", path, error),
                }
            None => println!("{}", text),
        }
    }

    if let Some(text) = output.stderr {
        match parsed_command.error_redirection {
            Some(path) =>
                match create_redirect_file(path, parsed_command.error_append) {
                    Ok(mut file) => {
                        if let Err(error) = writeln!(file, "{}", text) {
                            eprintln!("failed to write error to {}: {}", path, error);
                        }
                    }
                    Err(error) => eprintln!("failed to open error file {}: {}", path, error),
                }
            None => eprintln!("{}", text),
        }
    }
}
