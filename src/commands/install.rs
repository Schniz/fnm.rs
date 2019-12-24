use crate::config::FnmConfig;
use crate::downloader::{install_node_dist, Error};
use crate::remote_node_index;
use crate::version::UserVersion;
use colored::Colorize;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Install {
    pub version: UserVersion,
}

impl super::command::Command for Install {
    type Error = Error;

    fn apply(self, config: FnmConfig) -> Result<(), Self::Error> {
        let version = if let UserVersion::Semver(actual_version) = self.version {
            actual_version
        } else {
            let available_versions: Vec<_> = remote_node_index::list(&config.node_dist_mirror)
                .unwrap()
                .drain(..)
                .map(|x| x.version)
                .collect();
            self.version
                .to_version(&available_versions)
                .expect("Can't find requested Node version")
                .clone()
        };
        let version_str = format!("Node {}", &version);
        println!("Installing {}", version_str.cyan());
        install_node_dist(
            &version,
            &config.node_dist_mirror,
            config.installations_dir(),
        )?;

        Ok(())
    }

    fn handle_error(err: Self::Error) {
        dbg!(err);
    }
}
