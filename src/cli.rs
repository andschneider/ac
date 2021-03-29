use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ac", about = "andrew's CLI")]
pub struct Options {
    #[structopt(subcommand)]
    pub cmd: Option<Subcommand>,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// some git helpers
    Git(GitArgs),
}

#[derive(Debug, StructOpt)]
pub struct GitArgs {
    #[structopt(subcommand)]
    pub(crate) remote: RemoteArgs,
}

#[derive(Debug, StructOpt)]
pub enum RemoteArgs {
    /// change that remote
    Remote {
        // TODO is there a way to make these mutual exclusive?
        #[structopt(short, long)]
        /// flip the remote
        flip: bool,
        #[structopt(long)]
        /// set the remote to SSH
        to_ssh: bool,
        #[structopt(long)]
        /// set the remote to HTTPS
        to_https: bool,
    },
}
