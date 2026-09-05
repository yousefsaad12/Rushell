use std::env;
pub fn run() -> String {
    let current_dir = env::current_dir().unwrap();
    current_dir.display().to_string()
}
