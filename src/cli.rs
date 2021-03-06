use crate::commands;
use crate::commands::command::Command;
use crate::config::FnmConfig;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    /// List all remote Node.js versions
    #[structopt(name = "ls-remote")]
    LsRemote(commands::ls_remote::LsRemote),

    /// List all local Node.js versions
    #[structopt(name = "ls")]
    LsLocal(commands::ls_local::LsLocal),

    /// Install a new Node.js version
    #[structopt(name = "install")]
    Install(commands::install::Install),

    /// Change Node.js version
    #[structopt(name = "use")]
    Use(commands::r#use::Use),

    /// Print and setup required environment variables for fnm
    #[structopt(name = "env")]
    Env(commands::env::Env),

    /// Create completions file
    #[structopt(name = "completions")]
    Completions(commands::completions::Completions),

    /// alias a version to a common name
    #[structopt(name = "alias")]
    Alias(commands::alias::Alias),

    /// set a version as the default version
    #[structopt(name = "default")]
    Default(commands::default::Default),

    /// The current version
    #[structopt(name = "current")]
    Current(commands::current::Current),

    /// Run a command in fnm context
    #[structopt(name = "exec")]
    Exec(commands::exec::Exec),

    /// Uninstall a Node version (with its globally installed modules)
    #[structopt(name = "uninstall")]
    Uninstall(commands::uninstall::Uninstall),
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
            Self::Alias(cmd) => cmd.call(config),
            Self::Default(cmd) => cmd.call(config),
            Self::Current(cmd) => cmd.call(config),
            Self::Exec(cmd) => cmd.call(config),
            Self::Uninstall(cmd) => cmd.call(config),
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
