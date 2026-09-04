mod commands;
mod completion;
mod external;
mod repl;
mod tokenizer;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    repl::run();
}
