use std::process::Command;
use std::fs;

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let parent_dir = current_dir.parent().unwrap();
    let program_dir = std::env::current_exe().unwrap();
    let directories = fs::read_dir(parent_dir).unwrap();

    for dir in directories {
        let dir = dir.unwrap();
        let path = dir.path();

        if path.is_dir() && path != program_dir.parent().unwrap() {
            let output = Command::new("cargo")
                .arg("clean")
                .current_dir(&path)
                .output()
                .expect("Failed to run cargo clean");

            println!("{:?}", output);
        }
    }
}
