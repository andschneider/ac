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
    return stdout.to_string();
}
pub fn parse_remote(input: &str) -> Remote {
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
        ssh,
        repo_name: repo.to_string(),
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
        return format!("git@github.com:{}/{}.git", self.username, self.repo_name);
    }
    fn https_url(&self) -> String {
        return format!(
            "https://github.com/{}/{}.git",
            self.username, self.repo_name
        );
    }
    #[allow(dead_code)]
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
    pub fn flip_url(&self) {
        // git remote set-url origin git@github.com:USERNAME/REPOSITORY.git
        let url = self.create_url_opposite();
        let status = Remote::set_url(url).expect("failed to change remote");

        println!("set url exited with: {}", status);
        assert!(status.success());
    }

    // TODO error handling here?
    fn set_url(url: String) -> std::io::Result<ExitStatus> {
        Command::new("git")
            .arg("remote")
            .arg("set-url")
            .arg("origin")
            .arg(url)
            .status()
    }
}
