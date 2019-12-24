use super::shell::Shell;
use std::path::PathBuf;

#[derive(Debug)]
pub struct WindowsCmd;

impl Shell for WindowsCmd {
    fn into_structopt_shell(&self) -> structopt::clap::Shell {
        structopt::clap::Shell::PowerShell
    }

    fn path(&self, path: &PathBuf) -> String {
        format!("SET PATH={};%PATH%", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("SET {}={}", name, value)
    }

    fn use_on_cd(&self) -> String {
        "".into()
    }
}
