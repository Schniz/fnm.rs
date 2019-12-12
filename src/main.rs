mod archive;
mod commands;
mod config;
mod downloader;
mod remote_node_index;
mod system_info;
mod version;

use clap::Clap;
use commands::command::Command;
use config::FnmConfig;

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(name = "ls-remote", about = "List all remote Node.js versions")]
    LsRemote(commands::ls_remote::LsRemote),
    #[clap(name = "ls", about = "List all local Node.js versions")]
    LsLocal(commands::ls_local::LsLocal),
    #[clap(name = "install", about = "Install a new Node.js version")]
    Install(commands::install::Install),
}

impl SubCommand {
    pub fn call(self, config: FnmConfig) {
        match self {
            Self::LsLocal(cmd) => cmd.call(config),
            Self::LsRemote(cmd) => cmd.call(config),
            Self::Install(cmd) => cmd.call(config),
        }
    }
}

#[derive(Clap, Debug)]
#[clap(name = "fnm")]
struct Cli {
    #[clap(flatten)]
    config: FnmConfig,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() {
    let value = Cli::parse();
    value.subcmd.call(value.config);
}
