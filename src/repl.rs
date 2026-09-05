use crate::commands::{cd, echo, exit, pwd, type_cmd};
use crate::completion;
use crate::external::{executor, finder};
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

        let mut parts = tokens.iter();
        let command = parts.next().map(String::as_str).unwrap_or("");
        let args: Vec<&str> = parts.map(String::as_str).collect();
        match command {
            "echo" => echo::run(&args),
            "exit" => exit::run(),
            "type" => type_cmd::run(args.first().copied().unwrap_or("")),
            "pwd" => pwd::run(),
            "cd" => cd::run(&args),
            "" => {}
            _ => match finder::find_exe(command) {
                Some(path) => executor::run(&path, command, args),
                None => println!("{}: command not found", command),
            },
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
