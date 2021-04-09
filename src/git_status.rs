use std::process::Command;

/// Gets the `git status` of a repo, which is specified by its path.
// TODO pass in a filepath object instead of string?
pub fn get_git_status(dir: &str) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .arg("status")
        .arg("--porcelain=v1")
        .output()
        .expect("git status failed");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stdout = stdout.trim();
    stdout.to_string()
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
/// unpushed commits vs.  uncommitted changes.
fn parse_git_status(status: &str) -> bool {
    // println!("{}", status.len());
    !status.is_empty()
    // return status.len() != 0;
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
