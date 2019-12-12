use std::path::Path;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

pub trait Extract {
    fn extract_into<P: AsRef<Path>>(self, path: P) -> Result<(), Error>;
}
