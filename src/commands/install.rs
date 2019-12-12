use crate::config::FnmConfig;
use crate::downloader::{install_node_dist, Error};
use crate::version::Version;
use clap::Clap;
use colored::*;

#[derive(Clap, Debug)]
pub struct Install {
    version: Version,
}

impl super::command::Command for Install {
    type Error = Error;

    fn apply(self, config: FnmConfig) -> Result<(), Self::Error> {
        let version_str = format!("Node {}", &self.version);
        println!("Installing {}", version_str.cyan());
        install_node_dist(
            &self.version,
            &config.node_dist_mirror,
            config.installations_dir(),
        )?;

        Ok(())
    }

    fn handle_error(err: Self::Error) {
        dbg!(err);
    }
}
