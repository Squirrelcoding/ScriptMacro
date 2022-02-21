use std::process::Command;

pub fn run(m: &str) {
    Command::new("sh").arg(m).output().unwrap();
}
