use crate::system_info::{HostArch, HostPlatform};
use crate::version::Version;
use serde::Deserialize;

mod lts_status {
    use serde::{Deserialize, Deserializer};

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[serde(untagged)]
    enum LtsStatus {
        Nope(bool),
        Yes(String),
    }

    impl Into<Option<String>> for LtsStatus {
        fn into(self) -> Option<String> {
            match self {
                Self::Nope(_) => None,
                Self::Yes(x) => Some(x),
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(LtsStatus::deserialize(deserializer)?.into())
    }
}

#[derive(Deserialize, Debug)]
pub struct IndexedNodeVersion {
    pub version: Version,
    #[serde(with = "lts_status")]
    pub lts: Option<String>,
    pub date: chrono::NaiveDate,
    pub files: Vec<String>,
}

impl IndexedNodeVersion {
    pub fn download_url(&self, base_url: &reqwest::Url) -> reqwest::Url {
        base_url
            .join(&format!("{}/", self.version))
            .unwrap()
            .join(&format!(
                "node-{node_ver}-{platform}-{arch}.tar.xz",
                node_ver = &self.version,
                platform = HostPlatform::default(),
                arch = HostArch::default(),
            ))
            .unwrap()
    }
}

pub fn uncompressed_archive(
    response: reqwest::Response,
) -> tar::Archive<xz2::read::XzDecoder<reqwest::Response>> {
    let xz_stream = xz2::read::XzDecoder::new(response);
    tar::Archive::new(xz_stream)
}

pub fn list(base_url: &reqwest::Url) -> Result<Vec<IndexedNodeVersion>, reqwest::Error> {
    let index_json_url = format!("{}/index.json", base_url);
    let value: Vec<IndexedNodeVersion> = reqwest::get(&index_json_url)?.json()?;
    Ok(value)
}

pub fn print_stuff() -> Result<(), reqwest::Error> {
    let value: Vec<IndexedNodeVersion> =
        reqwest::get("https://nodejs.org/dist/index.json")?.json()?;

    let xxx: Vec<_> = value.iter().filter(|x| x.lts.is_some()).collect();
    let url = reqwest::Url::parse("https://nodejs.org/dist/").unwrap();

    let url = xxx.first().unwrap().download_url(&url);
    dbg!(&url);

    println!("a");
    let file_response = reqwest::get(url)?;
    let mut tar_stream = uncompressed_archive(file_response);
    println!("got tar stream");
    tar_stream.unpack("/tmp/").ok();
    println!("unpacked!");

    Ok(())
}
