use crate::version::Version;
use reqwest::Url;
use std::path::Path;
use std::path::PathBuf;
use tar::Archive;
use xz2::read::XzDecoder;

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    IoError(std::io::Error),
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

fn download_url(base_url: &Url, version: &Version) -> Url {
    use crate::system_info::{HostArch, HostPlatform};
    base_url
        .join(&format!("{}/", version))
        .unwrap()
        .join(&format!(
            "node-{node_ver}-{platform}-{arch}.tar.xz",
            node_ver = &version,
            platform = HostPlatform::default(),
            arch = HostArch::default(),
        ))
        .unwrap()
}

/// Install a Node package
pub fn install_node_dist<P: AsRef<Path>>(
    version: &Version,
    node_dist_mirror: &Url,
    installations_dir: P,
) -> Result<(), Error> {
    let url = download_url(node_dist_mirror, version);
    let response = reqwest::get(url)?;

    if response.status() == 404 {
        return Err(Error::VersionNotFound);
    }

    let mut installation_dir = PathBuf::from(installations_dir.as_ref());
    installation_dir.push(version.v_str());

    let xz_stream = XzDecoder::new(response);
    let mut tar_archive = Archive::new(xz_stream);
    tar_archive.unpack(&installation_dir)?;

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
        location_path.push("bin");
        location_path.push("node");

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
