mod commands;
mod completion;
mod external;
mod parser;
mod repl;
mod tokenizer;
mod redirect_io;
mod builtin;
mod output_handler;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    repl::run();
}
