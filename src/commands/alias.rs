use super::command::Command;
use crate::alias::create_alias;
use crate::config::FnmConfig;
use crate::installed_versions;
use crate::user_version::UserVersion;
use snafu::{OptionExt, ResultExt, Snafu};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Alias {
    pub(crate) to_version: UserVersion,
    pub(crate) name: String,
}

impl Command for Alias {
    type Error = Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error> {
        let all_versions =
            installed_versions::list(config.installations_dir()).context(VersionListingError)?;

        let to_version = self
            .to_version
            .to_version(&all_versions)
            .context(VersionNotFound {
                version: self.to_version,
            })?;

        create_alias(&config, &self.name, to_version).context(CantCreateSymlink)?;

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Can't create symlink for alias: {}", source))]
    CantCreateSymlink { source: std::io::Error },
    #[snafu(display("Can't list local installed versions: {}", source))]
    VersionListingError { source: installed_versions::Error },
    #[snafu(display("Version {} not found locally", version))]
    VersionNotFound { version: UserVersion },
}
