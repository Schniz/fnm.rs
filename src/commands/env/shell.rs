use std::fmt::Debug;
use std::path::PathBuf;

pub trait Shell: Debug {
    fn path(&self, path: &PathBuf) -> String;
}

#[derive(Debug)]
pub struct Bash;

impl Shell for Bash {
    fn path(&self, path: &PathBuf) -> String {
        format!("export PATH={}/bin:$PATH", path.to_str().unwrap())
    }
}

#[derive(Debug)]
pub struct Zsh;

impl Shell for Zsh {
    fn path(&self, path: &PathBuf) -> String {
        format!("export PATH={}/bin:$PATH", path.to_str().unwrap())
    }
}

#[derive(Debug)]
pub struct WindowsCmd;

impl Shell for WindowsCmd {
    fn path(&self, path: &PathBuf) -> String {
        format!("set PATH={};%PATH%", path.to_str().unwrap())
    }
}
