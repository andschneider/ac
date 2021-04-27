use git_url_parse::{GitUrl, Scheme};
use std::process::{Command, ExitStatus, Stdio};

#[derive(PartialEq, Default, Clone, Debug)]
pub struct Remote {
    remote: GitUrl,
    raw: String,
}

impl Remote {
    pub fn new() -> Self {
        let input = Remote::get_git_url();
        let git_remote = Remote::parse_remote(&input);
        Remote {
            raw: input,
            remote: git_remote,
        }
    }

    fn get_git_url() -> String {
        let output = Command::new("git")
            .arg("remote")
            .arg("get-url")
            .arg("--all")
            .arg("origin")
            .stdout(Stdio::piped())
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        stdout.trim().to_string()
    }

    fn parse_remote(input: &str) -> GitUrl {
        // TODO better error handling
        GitUrl::parse(input).expect("could not parse git remote")
    }

    fn ssh_url(&self) -> String {
        format!(
            "git@github.com:{}/{}.git",
            self.remote.owner.as_ref().unwrap(),
            self.remote.name
        )
    }
    fn https_url(&self) -> String {
        format!(
            "https://github.com/{}/{}.git",
            self.remote.owner.as_ref().unwrap(),
            self.remote.name
        )
    }

    pub fn flip_url(&self) {
        let url = match self.remote.scheme {
            Scheme::Ssh => self.https_url(),
            Scheme::Https => self.ssh_url(),
            Scheme::Http => self.ssh_url(),
            _ => {
                eprintln!("unsupported git scheme: {:?}", self.remote.scheme);
                eprintln!("defaulting to ssh");
                self.ssh_url()
            }
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

#[test]
fn test_parse() {
    let input = "git@github.com:andschneider/ac.git";
    let output = Remote::parse_remote(input);
    assert_eq!(output.owner.unwrap(), "andschneider");
    assert_eq!(output.scheme, Scheme::Ssh);
    assert_eq!(output.name, "ac");
}

#[test]
fn test_build_remote() {
    let remote = Remote::new();
    assert_eq!(remote.remote.owner.unwrap(), "andschneider");
    assert_eq!(remote.remote.name, "ac");
}
