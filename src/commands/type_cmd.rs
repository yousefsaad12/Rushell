use crate::external::finder::find_exe;

pub fn run(cmd: &str) {
    if cmd == "echo" || cmd == "exit" || cmd == "type" || cmd == "pwd" {
        println!("{} is a shell builtin", cmd)
    } else {
        match find_exe(cmd) {
            Some(path) => println!("{} is {}", cmd, path.display()),
            None => println!("{}: not found", cmd),
        }
    }
}
