use crate::external::finder;
use rustyline::completion::Pair;

const BUILTINS: [&str; 5] = ["cd", "echo", "exit", "pwd", "type"];

pub fn complete(prefix: &str) -> Vec<Pair> {
    let mut commands: Vec<String> = BUILTINS
        .iter()
        .filter(|builtin| builtin.starts_with(prefix))
        .map(|builtin| (*builtin).to_string())
        .collect();
    commands.extend(finder::find_executables(prefix));
    commands.sort();
    commands.dedup();

    commands
        .into_iter()
        .map(|command| Pair {
            display: command.clone(),
            replacement: format!("{command} "),
        })
        .collect()
}
