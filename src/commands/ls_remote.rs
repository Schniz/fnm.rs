use crate::config::FnmConfig;
use crate::remote_node_index;
use clap::Clap;

#[derive(Clap, Debug)]
pub struct LsRemote {}

impl super::command::Command for LsRemote {
    type Error = Error;

    fn apply(self, config: FnmConfig) -> Result<(), Self::Error> {
        let mut all_versions = remote_node_index::list(&config.node_dist_mirror)?;
        all_versions.sort();

        for version in all_versions {
            print!("{}", version.version);
            if let Some(lts) = &version.lts {
                print!(" ({})", lts);
            }
            println!("");
        }

        Ok(())
    }

    fn handle_error(err: Self::Error) {
        println!("An error has occured! {:?}", err);
    }
}

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::HttpError(err)
    }
}
