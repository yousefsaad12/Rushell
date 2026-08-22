use std::fs::OpenOptions;
use std::io::{ self, Write };

use crate::commands::{ cd, echo, exit, pwd, type_cmd };
use crate::external::{ executor, finder };
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

        let output: Option<String> = match command {
            "echo" => Some(echo::run(args)),

            "exit" => exit::run(),

            "type" => { Some(type_cmd::run(args.first().copied().unwrap_or(""))) }

            "pwd" => Some(pwd::run()),

            "cd" => {
                cd::run(args);
                None
            }

            "" => None,

            _ => {
                match finder::find_exe(command) {
                    Some(path) => {
                        executor::run(
                            &path,
                            command,
                            args,
                            parsed_command.output_redirection.as_deref(),
                            parsed_command.error_redirection.as_deref()
                        );

                        None
                    }

                    None => { Some(format!("{}: command not found", command)) }
                }
            }
        };

        if let Some(text) = output {
            match parsed_command.output_redirection.as_deref() {
                Some(path) => {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(path)
                        .expect("failed to open output file");

                    writeln!(file, "{}", text).expect("failed to write output");
                }

                None => {
                    println!("{}", text);
                }
            }
        }
    }
}
