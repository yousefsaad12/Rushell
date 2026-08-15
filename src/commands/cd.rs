use std::env;
pub fn run(args: &[&str]) {
    if args.len() == 0 {
        return;
    }

    if args.len() > 1 {
        println!("cd: too many arguments");
        return;
    }

    let new_path = if args[0] == "~" {
        std::env::home_dir().unwrap()
    } else {
        let current_dir = env::current_dir().unwrap();
        current_dir.join(args[0])
    };

    if new_path.exists() {
        env::set_current_dir(new_path).unwrap();
    } else {
        println!("cd: {}: No such file or directory", args[0]);
    }
}
