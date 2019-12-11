use crate::config::FnmConfig;
use crate::version::Version;
use clap::Clap;
use colored::*;
use crate::downloader::{install_node_dist, Error};

#[derive(Clap, Debug)]
pub struct Install {
    version: Version
}

impl super::command::Command for Install {
    type Error = Error;

    fn apply(self, config: FnmConfig) -> Result<(), Self::Error> {
        install_node_dist(&self.version, &config.node_dist_mirror, config.installations_dir())?;

        Ok(())
    }

    fn handle_error(err: Self::Error) {
        dbg!(err);
    }
}
