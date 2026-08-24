use std::io::{ self, Write };

use crate::builtin::{ execute_builtin, execute_external };
use crate::output_handler::handle_output;
use crate::parser::redirection::parse_redirection;
use crate::tokenizer::tokenize;

pub fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let line = line.trim();

        let tokens = tokenize(line);

        if tokens.is_empty() {
            continue;
        }

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
                parsed_command.error_append
            )
        };

        handle_output(output, &parsed_command);
    }
}
