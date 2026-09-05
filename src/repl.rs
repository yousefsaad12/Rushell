use crate::completion;
use crate::execution::{execute_builtin, execute_external, handle};
use crate::parser::redirection::parse_redirection;
use crate::tokenizer::tokenize;
use rustyline::error::ReadlineError;

pub fn run() {
    let mut editor = completion::new_editor().expect("failed to initialize line editor");

    loop {
        let line = match editor.readline("$ ") {
            Ok(line) => {
                if !line.trim().is_empty() {
                    let _ = editor.add_history_entry(line.as_str());
                }
                line
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                break;
            }
            Err(error) => {
                eprintln!("readline error: {error}");
                break;
            }
        };
        let line = line.trim();

        let tokens = tokenize(line);

        let parsed_command = parse_redirection(&tokens);

        let command = parsed_command.command;
        let args = &parsed_command.args;

        let output = match execute_builtin(command, args) {
            Some(output) => output,
            None => execute_external(
                command,
                args,
                parsed_command.output_redirection,
                parsed_command.output_append,
                parsed_command.error_redirection,
                parsed_command.error_append,
            ),
        };

        handle(output, &parsed_command);
    }
}
