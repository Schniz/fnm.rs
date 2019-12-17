use super::command::Command;
use super::install::Install;
use crate::config::FnmConfig;
use crate::version::UserVersion;
use clap::Clap;
use colored::Colorize;

#[derive(Clap, Debug)]
pub struct Use {
    version: UserVersion,
    /// Install the version if it isn't installed yet
    #[clap(long = "install-if-missing")]
    install_if_missing: bool,
}

impl Command for Use {
    type Error = Error;

    fn apply(self, config: FnmConfig) -> Result<(), Self::Error> {
        let all_versions = crate::installed_versions::list(config.installations_dir())?;
        let current_version = self.version.to_version(&all_versions);

        match current_version {
            Some(version) => {
                let version_path = {
                    let mut path = config.installations_dir();
                    path.push(version.to_string());
                    path.push("installation");
                    path
                };

                println!("Using Node {}", version.to_string().cyan());

                let multishell_path = config
                    .multishell_path
                    .expect("fnm isn't set up. Have you tried running `fnm env`?");

                std::fs::remove_file(&multishell_path)?;
                crate::fs::symlink_dir(version_path, &multishell_path)?;
            }
            None => {
                if !self.install_if_missing {
                    Err(Self::Error::CantFindVersion(self.version))?;
                } else {
                    Install {
                        version: self.version,
                    }
                    .apply(config)?;
                }
            }
        };

        Ok(())
    }

    fn handle_error(err: Self::Error) {
        println!("An error has occured! {:?}", err);
    }
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    InstallError(<Install as Command>::Error),
    VersionListingError(crate::installed_versions::Error),
    CantFindVersion(UserVersion),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<<Install as Command>::Error> for Error {
    fn from(err: <Install as Command>::Error) -> Self {
        Self::InstallError(err)
    }
}

impl From<crate::installed_versions::Error> for Error {
    fn from(err: crate::installed_versions::Error) -> Self {
        Self::VersionListingError(err)
    }
}
