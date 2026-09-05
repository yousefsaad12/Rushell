mod commands;
mod completion;
mod execution;
mod external;
mod parser;
mod redirection;
mod repl;
mod tokenizer;

fn main() {
    repl::run();
}
