mod infer_shell;
mod shell;

use self::shell::Shell;
use super::command::Command;
use crate::config::FnmConfig;
use clap::Clap;
use std::fmt::Debug;
use crate::fs::symlink_dir;

#[derive(Clap, Debug)]
pub struct Env {}

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
        let maybe_shell: Option<Box<dyn Shell>> = if cfg!(windows) {
            Some(Box::from(self::shell::WindowsCmd))
        } else {
            use self::infer_shell::infer_shell;
            infer_shell()
        };
        let multishell_path = make_symlink(&config);
        let shell = maybe_shell.expect("Can't infer shell!");
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

#[cfg(windows)]
fn infer_shell() -> Option<Box<WindowsCmd>> {
    Box::from(WindowsCmd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        let config = FnmConfig::default();
        Env {}.call(config);
    }
}
