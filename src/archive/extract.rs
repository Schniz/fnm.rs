use std::path::Path;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ZipError(zip::result::ZipError),
    HttpError(reqwest::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Self::ZipError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::HttpError(err)
    }
}

pub trait Extract {
    fn extract_into<P: AsRef<Path>>(self, path: P) -> Result<(), Error>;
}