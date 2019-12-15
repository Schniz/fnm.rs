use std::fmt::Debug;
use std::path::PathBuf;

pub trait Shell: Debug {
    fn path(&self, path: &PathBuf) -> String;
    fn set_env_var(&self, name: &str, value: &str) -> String;
    fn use_on_cd(&self) -> String;
}

#[cfg(windows)]
pub const AVAILABLE_SHELLS: [&'static str; 1] = ["cmd"];

#[cfg(unix)]
pub const AVAILABLE_SHELLS: [&'static str; 3] = ["bash", "zsh", "fish"];

impl std::str::FromStr for Box<dyn Shell> {
    type Err = String;

    fn from_str(s: &str) -> Result<Box<dyn Shell>, Self::Err> {
        match s {
            "cmd" => Ok(Box::from(super::windows_cmd::WindowsCmd)),
            "zsh" => Ok(Box::from(super::zsh::Zsh)),
            "bash" => Ok(Box::from(super::bash::Bash)),
            "fish" => Ok(Box::from(super::fish::Fish)),
            shell_type => Err(format!("I don't know the shell type of {:?}", shell_type)),
        }
    }
}
