use clap::{ArgMatches, Shell};
use std::io;

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
    // dir is guaranteed to unwrap since its a required arg
    let dir = args.value_of("dir").unwrap();
    let modified = args.is_present("modified");
    git_status::check_git_dirs(dir, modified);
}

fn run() {
    // let matches = cli::build_cli();
    let m = cli::build_cli().get_matches();
    match m.subcommand() {
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
    let matches = cli::build_cli().get_matches();

    if matches.is_present("generate-zsh-completions") {
        let outdir = match std::env::var_os("OUT_DIR") {
            None => return,
            Some(outdir) => outdir,
        };
        println!("generating completions to: {:?}", outdir);
        // cli::build_cli().gen_completions("ac", Shell::Zsh, outdir);
        cli::build_cli().gen_completions_to("ac", Shell::Zsh, &mut io::stdout());
        std::process::exit(0);
    }
    run();
}
