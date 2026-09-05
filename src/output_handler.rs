use std::io::Write;

use crate::redirect_io::create_redirect_file;
use crate::builtin::CommandOutput;
use crate::parser::redirection::ParsedCommand;

pub fn handle_output(output: CommandOutput, parsed_command: &ParsedCommand) {
    if let Some(text) = output.stdout {
        match parsed_command.output_redirection{
            Some(path) => {
                let mut file = create_redirect_file(path, parsed_command.output_append)
                    .expect("failed to open output file");

                writeln!(file, "{}", text).expect("failed to write output");
            }

            None => {
                println!("{}", text);
            }
        }
    }

    
    if let Some(text) = output.stderr {
        match parsed_command.error_redirection{
            Some(path) => {
                let mut file = create_redirect_file(path, parsed_command.error_append)
                    .expect("failed to open error file");

                writeln!(file, "{}", text).expect("failed to write error");
            }

            None => {
                eprintln!("{}", text);
            }
        }
    }
}
