use crate::external::finder::find_exe;

pub fn run(cmd: &str) -> String {
    if cmd == "echo" || cmd == "exit" || cmd == "type" || cmd == "pwd" {
        format!("{} is a shell builtin", cmd)
    } else {
        match find_exe(cmd) {
            Some(path) => format!("{} is {}", cmd, path.display()),
            None => format!("{}: not found", cmd),
        }
    }
}
