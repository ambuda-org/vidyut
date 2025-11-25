use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn find_git_dir_manually(start_path: &Path) -> Option<PathBuf> {
    let mut current_path = start_path.to_owned();

    loop {
        // Check if ".git" exists in the current directory
        let git_path = current_path.join(".git");
        if fs::metadata(&git_path).is_ok() {
            // It exists, return the path
            return Some(git_path);
        }

        // Check if the current directory is the root of the filesystem
        if !current_path.pop() {
            // Reached the root directory without finding .git
            return None;
        }
    }
}

pub fn find_src_root() -> Option<PathBuf> {
    match env::current_exe() {
        Ok(mut exe_path) => {
            // Get the directory containing the executable
            exe_path.pop();

            return match find_git_dir_manually(&exe_path) {
                Some(mut git_dir) => {
                    println!(".git directory found at: {}", git_dir.display());
                    git_dir.pop();
                    Some(git_dir)
                }
                None => {
                    println!("Could not find .git directory in the path hierarchy.");
                    None
                }
            };
        }
        Err(e) => eprintln!("Failed to get current exe path: {}", e),
    }
    None
}

// Dummy
#[allow(dead_code)]
fn main() {}
