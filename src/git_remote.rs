use std::process::{Command, ExitStatus, Stdio};

pub fn get_git_url() -> String {
    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("--all")
        .arg("origin")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stdout = stdout.trim();
    stdout.to_string()
}
pub fn parse_remote(input: &str) -> Remote {
    // check the first letter of the remote url
    let ssh = match input.chars().next().unwrap() {
        'g' => true,
        'h' => false,
        _ => false, // TODO throw error
    };
    // TODO replace all of this with regex
    let reversed: String = input.chars().rev().collect();
    let first: Vec<&str> = reversed.split('.').collect();
    let second: Vec<&str> = first[1].split('/').collect();
    let repo = second[0];
    let repo: String = repo.chars().rev().collect();

    Remote {
        url: input.to_string(),
        ssh,
        repo_name: repo,
        username: "andschneider".to_string(), // hard coding for now
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct Remote {
    url: String,
    ssh: bool,
    repo_name: String,
    username: String,
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
        format!("git@github.com:{}/{}.git", self.username, self.repo_name)
    }
    fn https_url(&self) -> String {
        format!(
            "https://github.com/{}/{}.git",
            self.username, self.repo_name
        )
    }

    pub fn flip_url(&self) {
        let url = match self.ssh {
            true => self.https_url(),
            false => self.ssh_url(),
        };
        Remote::change_url(url);
    }
    pub fn to_ssh(&self) {
        Remote::change_url(self.ssh_url());
    }
    pub fn to_https(&self) {
        Remote::change_url(self.https_url());
    }

    fn change_url(url: String) {
        let status = Remote::set_url(url).expect("failed to change remote");
        println!("set url exited with: {}", status);
        assert!(status.success());
    }

    // TODO error handling here?
    fn set_url(url: String) -> std::io::Result<ExitStatus> {
        // git remote set-url origin git@github.com:USERNAME/REPOSITORY.git
        Command::new("git")
            .arg("remote")
            .arg("set-url")
            .arg("origin")
            .arg(url)
            .status()
    }
}
