use crate::git_status::check_git_dirs;
use clap::{crate_version, App, AppSettings, Arg, ArgGroup, ArgMatches};
// use std::error::Error;
// use std::option::Option;

mod cli;
mod git_remote;
mod git_status;

fn handle_git_remote_command(args: &ArgMatches) {
    let stdout = git_remote::get_git_url();
    let remote = git_remote::parse_remote(&stdout);
    if args.is_present("flip") {
        remote.flip_url();
    } else if args.is_present("to-ssh") {
        remote.to_ssh();
    } else if args.is_present("to-https") {
        remote.to_https();
    }
}
fn handle_git_status_command(args: &ArgMatches) {
    let dir = args.value_of("dir").unwrap();
    check_git_dirs(dir);
}

fn run() {
    let matches = App::new("ac")
        .about("andrew's CLI")
        .version(crate_version!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("git")
                .about("some git helpers")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("remote")
                        .about("change that remote")
                        .group(
                            ArgGroup::with_name("remote_group")
                                .args(&["flip", "to-ssh", "to-https"])
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("flip")
                                .short("f")
                                .long("flip")
                                .help("flip the remote"),
                        )
                        .arg(
                            Arg::with_name("to-ssh")
                                .short("s")
                                .long("to-ssh")
                                .help("change the remote to ssh"),
                        )
                        .arg(
                            Arg::with_name("to-https")
                                .short("h")
                                .long("to-https")
                                .help("change the remote to https"),
                        ),
                )
                .subcommand(
                    App::new("status").about("status of repos").arg(
                        Arg::with_name("dir")
                            .short("d")
                            .long("dir")
                            .takes_value(true)
                            .help("the dir to check")
                            .required(true),
                    ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("git", Some(git_matches)) => match git_matches.subcommand() {
            ("remote", Some(remote_matches)) => {
                handle_git_remote_command(remote_matches);
            }
            ("status", Some(status_matches)) => {
                handle_git_status_command(status_matches);
            }
            _ => unreachable!(),
        },
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}

fn main() {
    run();
}
