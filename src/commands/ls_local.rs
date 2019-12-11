use crate::config::FnmConfig;
use crate::version::Version;
use clap::Clap;
use colored::*;

#[derive(Clap, Debug)]
pub struct LsLocal {}

fn current_version(config: &FnmConfig) -> Option<Version> {
    let multishell_path = config.multishell_path.as_ref()?;
    let mut multishell_path = std::fs::canonicalize(multishell_path).ok()?;
    multishell_path.pop();
    let file_name = multishell_path.file_name()?.to_str()?;
    Version::parse(file_name).ok()
}

impl super::command::Command for LsLocal {
    type Error = Error;

    fn apply(self, config: FnmConfig) -> Result<(), Self::Error> {
        let base_dir = config.installations_dir();
        let mut versions: Vec<_> = std::fs::read_dir(&base_dir)?
            .filter_map(|x| {
                if let Ok(version_dir) = x {
                    let file_name = version_dir.file_name();
                    file_name.to_str().and_then(|x| Version::parse(x).ok())
                } else {
                    None
                }
            })
            .collect();
        versions.sort();

        let curr_version = current_version(&config);

        for version in versions {
            let version_str = format!("* {}", version);
            if curr_version == Some(version) {
                println!("{}", version_str.cyan());
            } else {
                println!("{}", version_str);
            }
        }
        Ok(())
    }

    fn handle_error(err: Self::Error) {
        println!("An error has occured! {:?}", err);
    }
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
