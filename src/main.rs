use crate::git_remote::Remote;
use crate::permissions::Permission;
use clap::ArgMatches;

mod cli;
mod git_remote;
mod git_status;
mod permissions;

fn handle_git_remote_command(args: &ArgMatches) {
    let remote = Remote::new();
    if args.is_present("flip") {
        remote.flip_url();
    } else if args.is_present("to-ssh") {
        remote.to_ssh();
    } else if args.is_present("to-https") {
        remote.to_https();
    }
}

fn handle_git_status_command(args: &ArgMatches) {
    // dir is guaranteed to unwrap since its a required arg
    let dir = args.value_of("dir").unwrap();
    let modified = args.is_present("modified");
    git_status::check_git_dirs(dir, modified);
}

fn handle_permissions_command(args: &ArgMatches) {
    // input is guaranteed to unwrap since its a required arg
    let input = args.value_of("permission").unwrap();
    let perm = match input.parse::<u16>() {
        Ok(i) => Permission::from_octal(i),
        Err(_) => Permission::from_symbolic(input.to_string()),
    };
    match perm {
        Ok(p) => {
            if args.is_present("octal") {
                println!("{}", p.to_octal());
            } else if args.is_present("symbolic") {
                println!("{}", p.to_symbolic());
            }
        }
        Err(e) => {
            eprintln!("could not load permission: {}", e);
            std::process::exit(1);
        }
    }
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
        ("permissions", Some(permissions)) => match permissions.subcommand() {
            ("convert", Some(convert)) => {
                handle_permissions_command(convert);
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
