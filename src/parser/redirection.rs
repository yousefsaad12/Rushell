pub struct ParsedCommand<'a> {
    pub command: &'a str,
    pub args: Vec<&'a str>,
    pub output_redirection: Option<&'a str>,
    pub error_redirection: Option<&'a str>,
}

pub fn parse_redirection<'a>(tokens: &'a [String]) -> ParsedCommand<'a> {
    let command = tokens[0].as_str();
    let mut args = Vec::new();
    let mut output_redirection = None;
    let mut error_redirection = None;

    let mut i = 1;

    while i < tokens.len() {
        if tokens[i] == ">" || tokens[i] == "1>" {
            if i + 1 < tokens.len() {
                output_redirection = Some(tokens[i + 1].as_str());
                i += 2;
            } else {
                eprintln!("Error: No file specified for redirection");
                break;
            }
        } 
        else if tokens[i] == "2>" 
        {
            if i + 1 < tokens.len() 
            {
                error_redirection = Some(tokens[i + 1].as_str());
                i += 2;
            } 
            else 
            {
                eprintln!("Error: No file specified for error redirection");
                break;
            }
        }

         else {
            args.push(tokens[i].as_str());
            i += 1;
        }
    }

    ParsedCommand { command, args, output_redirection, error_redirection }
}
