use crate::version::Version;
use std::path::Path;

pub fn list<P: AsRef<Path>>(installations_dir: P) -> Result<Vec<Version>, Error> {
    let mut vec = vec![];
    for result_entry in installations_dir.as_ref().read_dir()? {
        let entry = result_entry?;
        let path = entry.path();
        let filename = path
            .file_name()
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?
            .to_str()
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?;
        let version = Version::parse(filename)?;
        vec.push(version);
    }
    Ok(vec)
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    SemverError(semver::SemVerError),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<semver::SemVerError> for Error {
    fn from(err: semver::SemVerError) -> Self {
        Self::SemverError(err)
    }
}
