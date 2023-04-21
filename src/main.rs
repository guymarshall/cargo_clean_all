#![forbid(unsafe_code)]

use std::process::{Command, Output};
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::{Path, PathBuf};

fn main() {
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let parent_dir: &Path = current_dir.parent().unwrap();
    let program_dir: PathBuf = std::env::current_exe().unwrap();
    let directories: ReadDir = fs::read_dir(parent_dir).unwrap();

    for dir in directories {
        let dir: DirEntry = dir.unwrap();
        let path: PathBuf = dir.path();

        if path == current_dir || path == program_dir.parent().unwrap() {
            continue;
        }

        if path.is_dir() {
            let output: Output = Command::new("cargo")
                .arg("clean")
                .current_dir(&path)
                .output()
                .expect("Failed to run cargo clean");

            println!("Cleaning directory: {} | Status: {} | Stdout: {} | Stderr: {}",
                 path.display(),
                 output.status,
                 String::from_utf8_lossy(&output.stdout),
                 String::from_utf8_lossy(&output.stderr));
        }
    }
}
