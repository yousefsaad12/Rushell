pub fn tokenize(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut arg_start = false;

    for c in input.chars() {
        match c {
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
                arg_start = true;
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
                arg_start = true;
            }
            c if c.is_whitespace() && !in_single_quotes && !in_double_quotes => {
                if arg_start {
                    args.push(std::mem::take(&mut current_arg));
                    arg_start = false;
                }
            }
            _ => {
                current_arg.push(c);
                arg_start = true;
            }
        }
    }

    if arg_start {
        args.push(current_arg);
    }

    args
}
