use std::error::Error;
use std::process::{Command, Stdio};
use std::str::FromStr;

#[derive(PartialEq, Default, Clone, Debug)]
struct Remote {
    url: String,
    style: String,     // TODO use ssh or http enum?
    repo_name: String, // TODO what are the naming conventions?
    username: String,
}

impl FromStr for Remote {
    type Err = std::num::ParseIntError;

    // git@github.com:andschneider/ac.git
    // first: git, github.com:andschneider/ac.git
    // second: github.com, andschneider/ac.git
    // third: andschneider, ac.git
    fn from_str(output: &str) -> Result<Self, Self::Err> {
        // TODO need to check if it's ssh or https first
        let first: Vec<&str> = output.split("@").collect();
        let style = first[0];
        let url = first[1];
        let second: Vec<&str> = url.split(":").collect();
        let third: Vec<&str> = second[1].split("/").collect();
        Ok(Remote {
            url: url.to_string(),
            style: style.to_string(),
            repo_name: third[1].to_string(),
            username: third[0].to_string(),
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
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
    let remote = stdout.trim();
    println!("{}", remote);
    match Remote::from_str(&remote) {
        Ok(r) => {
            println!(
                r"Remote style: {}, url: {}, username: {}, repo name: {}",
                r.style, r.url, r.username, r.repo_name,
            );
            println!("https: https://github.com/{}/{}", r.username, r.repo_name);
            println!("ssh: git@github.com:{}/{}", r.username, r.repo_name);
        }
        Err(_) => {
            println!("{} is not a valid color hex code!", remote);
        }
    }

    Ok(())
}
