mod infer_shell;
mod shell;

use self::shell::{Shell, AVAILABLE_SHELLS};
use super::command::Command;
use crate::config::FnmConfig;
use crate::fs::symlink_dir;
use clap::Clap;
use std::fmt::Debug;

#[derive(Clap, Debug)]
pub struct Env {
    /// The shell syntax to use. Infers when missing.
    #[clap(long = "shell")]
    #[clap(raw(possible_values = "&AVAILABLE_SHELLS"))]
    shell: Option<Box<dyn Shell>>,
}

#[derive(Debug)]
pub enum Error {}

fn make_symlink(config: &FnmConfig) -> std::path::PathBuf {
    let temp_dir_name = format!(
        "fnm_multishell_{}",
        chrono::Utc::now().to_rfc3339().replace(":", "__")
    );

    let mut system_temp_dir = std::env::temp_dir();
    system_temp_dir.push(&temp_dir_name);

    symlink_dir(config.default_version_dir(), &system_temp_dir).expect("Can't create symlink!");

    system_temp_dir
}

impl Command for Env {
    type Error = Error;

    fn apply(self, config: FnmConfig) -> Result<(), Self::Error> {
        let shell: Box<dyn Shell> = match self.shell {
            Some(shell) => shell,
            None => {
                if cfg!(windows) {
                    Box::from(self::shell::WindowsCmd)
                } else {
                    use self::infer_shell::infer_shell;
                    infer_shell().expect("Can't infer shell!")
                }
            }
        };
        let multishell_path = make_symlink(&config);
        println!("{}", shell.path(&multishell_path));
        println!(
            "{}",
            shell.set_env_var("FNM_MULTISHELL_PATH", multishell_path.to_str().unwrap())
        );
        println!("{}", shell.use_on_cd());
        Ok(())
    }

    fn handle_error(err: Self::Error) {
        dbg!(err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        let config = FnmConfig::default();
        let shell: Box<dyn Shell> = if cfg!(windows) {
            Box::from(self::shell::WindowsCmd)
        } else {
            Box::from(self::shell::Bash)
        };
        Env { shell: Some(shell) }.call(config);
    }
}
