use crate::config::FnmConfig;
use crate::installed_versions;
use crate::system_version;
use crate::user_version::UserVersion;
use crate::version::Version;
use colored::Colorize;
use log::info;
use snafu::{ensure, ResultExt, Snafu};
use std::path::{Path, PathBuf};

pub struct ApplicableVersion {
    path: PathBuf,
    candidates: Vec<Version>,
}

impl ApplicableVersion {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn version(&self) -> &Version {
        self.candidates
            .iter()
            .max()
            .expect("candidates list to have contents")
    }

    pub fn candidates(&self) -> &Vec<Version> {
        &self.candidates
    }
}

pub fn choose_version_for_user_input<'a>(
    requested_version: &'a UserVersion,
    config: &'a FnmConfig,
) -> Result<Option<ApplicableVersion>, Error> {
    let all_versions =
        installed_versions::list(config.installations_dir()).context(VersionListing)?;

    let result = if let UserVersion::Full(Version::Bypassed) = requested_version {
        info!("Bypassing fnm: using {} node", "system".cyan());
        Some(ApplicableVersion {
            path: system_version::path(),
            candidates: vec![Version::Bypassed],
        })
    } else if let Some(alias_name) = requested_version.alias_name() {
        let alias_path = config.aliases_dir().join(&alias_name);
        ensure!(
            alias_path.exists(),
            CantFindVersion {
                requested_version: requested_version.clone()
            }
        );
        info!("Using Node for alias {}", alias_name.cyan());
        Some(ApplicableVersion {
            path: alias_path,
            candidates: vec![Version::Alias(alias_name)],
        })
    } else {
        let mut all_versions = all_versions;
        let applicable_versions: Vec<_> = all_versions
            .drain(..)
            .filter(|x| requested_version == x)
            .collect();

        match applicable_versions.iter().max() {
            None => None,
            Some(version) => {
                info!("Using Node {}", version.to_string().cyan());
                let path = config
                    .installations_dir()
                    .join(version.to_string())
                    .join("installation");

                Some(ApplicableVersion {
                    path,
                    candidates: applicable_versions,
                })
            }
        }
    };

    Ok(result)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Can't find requested version: {}", requested_version))]
    CantFindVersion { requested_version: UserVersion },
    #[snafu(display("Can't list local installed versions: {}", source))]
    VersionListing { source: installed_versions::Error },
}
