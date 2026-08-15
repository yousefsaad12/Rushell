use std::io::{ self, Write };
use crate::commands::{ echo, exit, type_cmd, cd, pwd };
use crate::external::{ finder, executor };
use crate::tokenizer::tokenize;

pub fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
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
            _ => {
                match finder::find_exe(command) {
                    Some(path) => executor::run(&path, command, args),
                    None => println!("{}: command not found", command),
                }
            }
        }
    }
}
