use crate::cli::{Options, RemoteArgs, Subcommand};
use std::error::Error;
use structopt::StructOpt;

mod cli;
mod git_remote;
mod git_status;

fn handle_git_remote_command(args: RemoteArgs) {
    let stdout = git_remote::get_git_url();
    let remote = git_remote::parse_remote(&stdout);
    // TODO handle more?
    match args {
        RemoteArgs::Remote { flip, .. } => match flip {
            true => {
                remote.flip_url();
            }
            false => {}
        },
    };
}

fn run(args: Options) {
    if let Some(subcommand) = args.cmd {
        match subcommand {
            Subcommand::Git(cfg) => {
                handle_git_remote_command(cfg.remote);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Options::from_args();
    // println!("{:?}", &args);
    run(args);
    Ok(())
}
