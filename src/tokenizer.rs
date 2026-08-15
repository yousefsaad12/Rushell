pub fn tokenize(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut arg_start = false;

    for c in input.chars() {
        match c {
            '\'' => {
                in_quotes = !in_quotes;
                arg_start = true;
            }
            c if c.is_whitespace() && !in_quotes => {
                if arg_start {
                    args.push(std::mem::take(&mut current_arg));
                    current_arg.clear();
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
