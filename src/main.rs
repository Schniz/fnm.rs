mod archive;
mod cli;
mod commands;
mod config;
mod downloader;
mod fs;
mod installed_versions;
mod remote_node_index;
mod shell;
mod system_info;
mod version;

fn main() {
    let value = crate::cli::parse();
    value.subcmd.call(value.config);
}
