use crate::archive;
use crate::archive::{Extract, TarXz, Zip};
use crate::version::Version;
use reqwest::Url;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    IoError(std::io::Error),
    ZipError(zip::result::ZipError),
    TarIsEmpty,
    VersionNotFound,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::HttpError(err)
    }
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

impl From<archive::Error> for Error {
    fn from(err: archive::Error) -> Self {
        match err {
            archive::Error::IoError(io_err) => Self::IoError(io_err),
        }
    }
}

#[cfg(unix)]
fn filename_for_version(version: &Version) -> String {
    use crate::system_info::{HostArch, HostPlatform};
    format!(
        "node-{node_ver}-{platform}-{arch}.tar.xz",
        node_ver = &version,
        platform = HostPlatform::default(),
        arch = HostArch::default(),
    )
}

#[cfg(windows)]
fn filename_for_version(version: &Version) -> String {
    format!(
        "node-{node_ver}-windows-{arch}.zip",
        node_ver = &version,
        arch = crate::system_info::HostArch::default(),
    )
}

fn download_url(base_url: &Url, version: &Version) -> Url {
    base_url
        .join(&format!("{}/", version))
        .unwrap()
        .join(&filename_for_version(version))
        .unwrap()
}

#[cfg(unix)]
pub fn extract_archive_into<P: AsRef<Path>>(
    path: P,
    response: reqwest::Response,
) -> Result<(), Error> {
    TarXz::new(response).extract_into(path)?;
    Ok(())
}

#[cfg(windows)]
pub fn extract_archive_into<P: AsRef<Path>>(
    path: P,
    mut response: reqwest::Response,
) -> Result<(), Error> {
    Zip::new(response).extract_into(path)?;
    Ok(())
}

/// Install a Node package
pub fn install_node_dist<P: AsRef<Path>>(
    version: &Version,
    node_dist_mirror: &Url,
    installations_dir: P,
) -> Result<(), Error> {
    let url = download_url(node_dist_mirror, version);
    println!("Going to call for {}", &url);
    let response = reqwest::get(url)?;

    if response.status() == 404 {
        return Err(Error::VersionNotFound);
    }

    let mut installation_dir = PathBuf::from(installations_dir.as_ref());
    installation_dir.push(version.v_str());
    extract_archive_into(&installation_dir, response)?;

    let installed_directory = std::fs::read_dir(&installation_dir)?
        .next()
        .ok_or(Error::TarIsEmpty)??;
    let installed_directory = installed_directory.path();

    let mut renamed_installation_dir = PathBuf::from(&installation_dir);
    renamed_installation_dir.push("installation");

    std::fs::rename(installed_directory, renamed_installation_dir)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::downloader::install_node_dist;
    use crate::version::Version;
    use pretty_assertions::assert_eq;
    use std::io::Read;
    use tempdir::TempDir;

    #[cfg(unix)]
    fn node_path<P: AsRef<Path>>(installation_dir: P) -> PathBuf {
        let mut pathbuf = PathBuf::from(installation_dir.as_ref());
        pathbuf.push("bin");
        pathbuf.push("node");
        pathbuf
    }

    #[cfg(windows)]
    fn node_path<P: AsRef<Path>>(installation_dir: P) -> PathBuf {
        let mut pathbuf = PathBuf::from(installation_dir.as_ref());
        pathbuf.push("node.cmd");
        pathbuf
    }

    #[test]
    fn test_installing_node_12() {
        let version = Version::parse("12.0.0").unwrap();
        let node_dist_mirror = Url::parse("https://nodejs.org/dist/").unwrap();
        let installations_dir = TempDir::new("node_12_installation").unwrap();
        install_node_dist(&version, &node_dist_mirror, &installations_dir)
            .expect("Can't install Node 12");

        let mut location_path = PathBuf::from(&installations_dir.path());
        location_path.push(version.v_str());
        location_path.push("installation");
        let location_path = node_path(location_path);

        let mut result = String::new();
        std::process::Command::new(location_path.to_str().unwrap())
            .arg("--version")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Can't find node executable")
            .stdout
            .expect("Can't find stdout")
            .read_to_string(&mut result)
            .expect("Failed reading stdout");
        assert_eq!(result.trim(), "v12.0.0");
    }
}
