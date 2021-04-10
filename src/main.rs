use clap::ArgMatches;

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
    git_status::check_git_dirs(dir);
}

fn run() {
    let matches = cli::create_app();
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
    // TODO add error handling to run
    run();
}
