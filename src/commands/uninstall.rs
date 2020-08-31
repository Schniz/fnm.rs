use super::command::Command;
use crate::choose_version_for_user_input::{
    choose_version_for_user_input, Error as ChooseVersionError,
};
use crate::config::FnmConfig;
use crate::outln;
use crate::user_version::UserVersion;
use crate::version::Version;
use colored::Colorize;
use log::info;
use snafu::{ensure, OptionExt, ResultExt, Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Uninstall {
    version: UserVersion,
}

impl Command for Uninstall {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let applicable_version = choose_version_for_user_input(&self.version, &config)
            .context(CantGetApplicableVersion)?
            .context(NoApplicableVersion {
                requested_version: self.version.clone(),
            })?;

        ensure!(
            applicable_version.candidates().len() < 2,
            AmbiguousDeletion {
                requested_version: self.version,
                candidates: applicable_version.candidates().clone()
            }
        );

        let delete_path =
            applicable_version
                .version()
                .root_path(&config)
                .context(UninstallableVersion {
                    version: applicable_version.version().clone(),
                })?;

        info!("Deleting directory {}", delete_path.display());
        std::fs::remove_dir_all(&delete_path).context(CantDeleteVersion)?;
        info!("Deleted directory {}", delete_path.display());

        match applicable_version.version() {
            Version::Bypassed => unreachable!(),
            v @ Version::Alias(_) => {
                outln!(config#Info, "Unaliased {}", v.v_str().cyan());
            }
            v @ Version::Semver(_) | v @ Version::Lts(_) => {
                outln!(config#Info, "Uninstalled version {}", v.v_str().cyan());
            }
        };

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{}", source))]
    CantGetApplicableVersion { source: ChooseVersionError },
    #[snafu(display("Can't find version to uninstall that matches {}", requested_version))]
    NoApplicableVersion { requested_version: UserVersion },
    #[snafu(display("version {} can't be uninstalled", format!("{}", version).italic()))]
    UninstallableVersion { version: Version },
    #[snafu(
        display(
            "requested version {} has too many candidates:\n{}\nPlease narrow your selection.",
            requested_version,
            candidates.iter().map(|x| format!("- {}", x)).collect::<Vec<_>>().join("\n")
        )
    )]
    AmbiguousDeletion {
        requested_version: UserVersion,
        candidates: Vec<Version>,
    },
    #[snafu(display(
        "Can't delete version: {}\nTry to remove it manually to avoid a malformed file system!",
        source
    ))]
    CantDeleteVersion { source: std::io::Error },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_missing_version() {
        let fnm_dir = tempfile::tempdir().unwrap();
        let config = FnmConfig {
            base_dir: Some(fnm_dir.path().to_path_buf()),
            ..FnmConfig::default()
        };

        let installed_result = Uninstall {
            version: UserVersion::from_str("v10").unwrap(),
        }
        .apply(&config);

        assert!(matches!(
            installed_result,
            Err(Error::NoApplicableVersion { .. })
        ));
    }

    #[test]
    fn test_ambiguous_deletion() {
        let fnm_dir = tempfile::tempdir().unwrap();
        let config = FnmConfig {
            base_dir: Some(fnm_dir.path().to_path_buf()),
            ..FnmConfig::default()
        };

        for version_str in ["10.11.0", "10.12.0"].iter() {
            crate::downloader::install_node_dist(
                &Version::parse(version_str).unwrap(),
                &config.node_dist_mirror,
                config.installations_dir(),
            )
            .unwrap();
        }

        let installed_result = Uninstall {
            version: UserVersion::from_str("v10").unwrap(),
        }
        .apply(&config);

        assert!(matches!(
            installed_result,
            Err(Error::AmbiguousDeletion { .. })
        ));
    }
}
