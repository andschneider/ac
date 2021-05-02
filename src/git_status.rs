use colored::*;
use std::fmt;
use std::path::Path;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

#[derive(Clone, Debug, PartialOrd, Eq, PartialEq)]
pub enum GitStatus {
    Aok,
    Modified,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct GitDir<'a> {
    path: &'a Path,
    status: GitStatus,
}

impl<'a> GitDir<'a> {
    pub fn new(path: &'a Path) -> Self {
        GitDir {
            path,
            status: GitStatus::Unknown,
        }
    }

    /// Gets the `git status` of a repo. It updates the status field by parsing the output with the
    /// `parse_git_status` function.
    fn get_git_status(&mut self) {
        let output = Command::new("git")
            .current_dir(self.path)
            .arg("status")
            .arg("--porcelain=v1")
            .output()
            .expect("git status failed");

        let status = String::from_utf8(output.stdout).unwrap(); // likely want to .trim() this
        self.status = GitDir::parse_git_status(status);
    }

    /// Determine if the status indicates the repository has been modified. Right now, it does this
    /// in a terrible way, by checking if the number of bytes of the string is greater than 0.
    ///
    /// In the future, I would like to add more intelligent parsing here, e.g. to determine if there are
    /// unpushed commits vs. uncommitted changes.
    fn parse_git_status(status: String) -> GitStatus {
        if status.is_empty() {
            return GitStatus::Aok;
        }
        GitStatus::Modified
    }
}

impl fmt::Display for GitDir<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // double unwrap to remove "" around the file name: "repo" => repo
        let dir_name = self.path.file_name().unwrap_or_default().to_str().unwrap();
        match self.status {
            GitStatus::Modified => {
                write!(f, "{} {}", "M".red(), dir_name)
            }
            GitStatus::Aok => {
                write!(f, "{} {}", "✓".green(), dir_name)
            }
            GitStatus::Unknown => write!(f, "{} {}", "U".yellow(), dir_name),
        }
    }
}

/// Checks if every directory located at the path specified is a git repo, and if it is, then checks
/// if there are any outstanding changes.
#[allow(clippy::if_same_then_else)]
pub fn check_git_dirs(dir: &str, modified_only: bool) {
    fn is_repo(entry: &DirEntry) -> bool {
        Path::new(entry.path()).join(".git").exists()
    }
    let walker = WalkDir::new(dir)
        .min_depth(1)
        .max_depth(1)
        .sort_by_file_name()
        .into_iter();
    for entry in walker.filter_entry(|e| is_repo(e)) {
        let dir_entry = entry.unwrap();
        let mut dir = GitDir::new(dir_entry.path());
        dir.get_git_status();
        if dir.status == GitStatus::Modified {
            println!("{}", dir);
        } else if !modified_only {
            // display unmodified repos as well
            println!("{}", dir);
        }
    }
}

#[test]
fn test_git_status() {
    let d = Path::new(".");
    let mut gd = GitDir::new(d);
    gd.get_git_status();
    println!("{:?}", gd.status);
    println!("{:?}", gd.path);
    println!("{}", gd);
}

#[test]
fn test_parse_git_status() {
    let status = "M src/main.rs\n\
    ?? src/git_status.rs";
    let s = GitDir::parse_git_status(status.to_string());
    assert_eq!(s, GitStatus::Modified);
}

#[test]
fn test_parse_git_status_blank() {
    let status = "";
    let s = GitDir::parse_git_status(status.to_string());
    assert_eq!(s, GitStatus::Aok);
}
