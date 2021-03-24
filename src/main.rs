use std::error::Error;
use std::process::{Command, Stdio};

#[derive(PartialEq, Default, Clone, Debug)]
struct Remote {
    url: String,
    ssh: bool,
    repo_name: String,
    username: String,
}

fn parse_remote(input: &str) -> Remote {
    // check the first letter of the remote url
    let ssh = match input.chars().nth(0).unwrap() {
        'g' => true,
        'h' => false,
        _ => false, // TODO throw error
    };
    // TODO replace all of this with regex
    let reversed: String = input.chars().rev().collect();
    let first: Vec<&str> = reversed.split(".").collect();
    let second: Vec<&str> = first[1].split("/").collect();
    let repo = second[0];
    let repo: String = repo.chars().rev().collect();

    Remote {
        url: input.to_string(),
        ssh: ssh,
        repo_name: repo.to_string(),
        username: "andschneider".to_string(), // hard coding for now
    }
}

#[test]
fn test_parse() {
    let input = "git@github.com:andschneider/ac.git";
    let output = parse_remote(input);
    println!("{}", output.username);
    assert_eq!(output.username, "andschneider");
    assert_eq!(output.url, input);
    assert_eq!(output.ssh, true);
    assert_eq!(output.repo_name, "ac");
}

impl Remote {
    fn ssh_url(&self) -> String {
        return format!("git@github.com:{}/{}.git", self.username, self.repo_name);
    }
    fn https_url(&self) -> String {
        return format!(
            "https://github.com/{}/{}.git",
            self.username, self.repo_name
        );
    }
    fn create_url(&self) -> String {
        match self.ssh {
            true => self.ssh_url(),
            false => self.https_url(),
        }
    }
    fn create_url_opposite(&self) -> String {
        match self.ssh {
            true => self.https_url(),
            false => self.ssh_url(),
        }
    }
}

fn get_git_url() -> String {
    let mut git = Command::new("git");
    let output = git
        .arg("remote")
        .arg("get-url")
        .arg("--all")
        .arg("origin")
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stdout = stdout.trim();
    return stdout.to_string();
}

fn set_git_url(url: &str) {
    // git remote set-url origin git@github.com:USERNAME/REPOSITORY.git
    let status = Command::new("git")
        .arg("remote")
        .arg("set-url")
        .arg("origin")
        .arg(url)
        .status()
        .expect("failed to execute process");

    println!("set url exited with: {}", status);
    assert!(status.success());
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = get_git_url();
    let remote = parse_remote(&stdout);
    let curr_url = remote.create_url();
    let new_url = remote.create_url_opposite();
    println!("{}", curr_url);
    println!("{}", new_url);
    set_git_url(&new_url);
    Ok(())
}
