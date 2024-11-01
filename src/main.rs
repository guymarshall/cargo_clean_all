#![forbid(unsafe_code)]

use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn main() {
    let current_directory: PathBuf = std::env::current_dir().unwrap();
    let parent_directory: &Path = current_directory.parent().unwrap();
    let program_directory: PathBuf = std::env::current_exe().unwrap();
    let directories: ReadDir = fs::read_dir(parent_directory).unwrap();

    directories.for_each(|directory: Result<DirEntry, Error>| {
        let directory: DirEntry = directory.unwrap();
        let path: PathBuf = directory.path();

        if path != current_directory && path != program_directory.parent().unwrap() && path.is_dir()
        {
            let output: Output = Command::new("cargo")
                .arg("clean")
                .current_dir(&path)
                .output()
                .expect("Failed to run cargo clean");

            match output.status.code() {
                Some(0) => println!("Cleaned {} successfully", path.display()),
                Some(101) => println!("No Cargo.toml file in {}", path.display()),
                _ => println!(
                    "Cleaning directory: {} | Status: {} | Stdout: {} | Stderr: {}",
                    path.display(),
                    output.status,
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ),
            }
        }
    });
}
