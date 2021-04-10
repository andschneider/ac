use colored::*;
use std::path::Path;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

fn is_repo(entry: &DirEntry) -> bool {
    let mut is_repo = false;
    if entry.file_type().is_dir() {
        is_repo = Path::new(entry.path()).join(".git").exists();
        // println!("{:?}", entry.path());
        // println!("{:?}", is_repo);
        // Don't need to use this, just check the path
        // `git -C <path> rev-parse` will
        // let output = Command::new("git")
        //     .arg("-C")
        //     .arg(entry.path())
        //     .arg("rev-parse")
        //     .status()
        //     .expect("git status failed");
        //
        // return output.success();
    }
    is_repo
}

/// Checks if every directory located at the path specified is a git repo, and if it is, then checks
/// if there are any outstanding changes. If the repo has changes, the path will be displayed
/// prefixed by a `M` and also will be displayed red. Otherwise the path will be displayed green.
pub fn check_git_dirs(dir: &str) {
    let walker = WalkDir::new(dir).min_depth(1).max_depth(1).into_iter();
    for entry in walker.filter_entry(|e| is_repo(e)) {
        let entry = entry.unwrap();
        let stdout = get_git_status(entry.path());
        if parse_git_status(stdout.trim()) {
            println!("{} {}", "M".red().bold(), entry.path().display());
        } else {
            println!("{} {}", "✓".green(), entry.path().display());
        }
    }
}

/// Gets the `git status` of a repo, which is specified by its path.
fn get_git_status(dir: &Path) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .arg("status")
        .arg("--porcelain=v1")
        .output()
        .expect("git status failed");

    String::from_utf8(output.stdout).unwrap() // likely want to .trim() this
}

#[test]
fn test_git_status() {
    let d = ".";
    let status = get_git_status(d);
    println!("{}", status);
}

/// Determine if the status indicates the repository has been modified. It will return false if
/// there are no changes, otherwise it will return true. Right now, it does this in a terrible way,
/// by checking if the number of bytes of the string is greater than 0.
///
/// In the future, I would like to add more intelligent parsing here, e.g. to determine if there are
/// unpushed commits vs. uncommitted changes.
fn parse_git_status(status: &str) -> bool {
    !status.is_empty()
}

#[test]
fn test_parse_git_status() {
    let status = "M src/main.rs\n\
    ?? src/git_status.rs";
    let s = parse_git_status(status);
    assert!(s);
}

#[test]
fn test_parse_git_status_blank() {
    let status = "";
    let s = parse_git_status(status);
    assert!(!s);
}
