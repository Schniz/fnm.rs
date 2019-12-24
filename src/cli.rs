use crate::commands;
use crate::commands::command::Command;
use crate::config::FnmConfig;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    #[structopt(name = "ls-remote", about = "List all remote Node.js versions")]
    LsRemote(commands::ls_remote::LsRemote),
    #[structopt(name = "ls", about = "List all local Node.js versions")]
    LsLocal(commands::ls_local::LsLocal),
    #[structopt(name = "install", about = "Install a new Node.js version")]
    Install(commands::install::Install),
    #[structopt(name = "use", about = "Change Node.js version")]
    Use(commands::r#use::Use),
    #[structopt(
        name = "env",
        about = "Print and setup required environment variables for fnm"
    )]
    Env(commands::env::Env),
    #[structopt(name = "completions", about = "Create completions file")]
    Completions(commands::completions::Completions),
}

impl SubCommand {
    pub fn call(self, config: FnmConfig) {
        match self {
            Self::LsLocal(cmd) => cmd.call(config),
            Self::LsRemote(cmd) => cmd.call(config),
            Self::Install(cmd) => cmd.call(config),
            Self::Env(cmd) => cmd.call(config),
            Self::Use(cmd) => cmd.call(config),
            Self::Completions(cmd) => cmd.call(config),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "fnm")]
pub struct Cli {
    #[structopt(flatten)]
    pub config: FnmConfig,
    #[structopt(subcommand)]
    pub subcmd: SubCommand,
}

pub fn parse() -> Cli {
    Cli::from_args()
}
