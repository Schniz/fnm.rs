use super::command::Command;
use crate::config::FnmConfig;
use crate::fs::symlink_dir;
use crate::shell::{infer_shell, Shell, AVAILABLE_SHELLS};
use std::fmt::Debug;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Env {
    /// The shell syntax to use. Infers when missing.
    #[structopt(long = "shell")]
    #[structopt(possible_values = AVAILABLE_SHELLS)]
    shell: Option<Box<dyn Shell>>,
}

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
    type Error = ();

    fn apply(self, config: FnmConfig) -> Result<(), Self::Error> {
        let shell: Box<dyn Shell> = self.shell.unwrap_or_else(&infer_shell);
        let multishell_path = make_symlink(&config);
        println!("{}", shell.path(&multishell_path));
        println!(
            "{}",
            shell.set_env_var("FNM_MULTISHELL_PATH", multishell_path.to_str().unwrap())
        );
        println!(
            "{}",
            shell.set_env_var("FNM_DIR", config.base_dir_with_default().to_str().unwrap())
        );
        println!(
            "{}",
            shell.set_env_var("FNM_LOGLEVEL", config.loglevel.into())
        );
        println!(
            "{}",
            shell.set_env_var("FNM_NODE_DIST_MIRROR", config.node_dist_mirror.as_str())
        );
        println!("{}", shell.use_on_cd());
        Ok(())
    }

    fn handle_error(_err: Self::Error) {
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke() {
        use crate::shell;
        let config = FnmConfig::default();
        let shell: Box<dyn Shell> = if cfg!(windows) {
            Box::from(shell::WindowsCmd)
        } else {
            Box::from(shell::Bash)
        };
        Env { shell: Some(shell) }.call(config);
    }
}
