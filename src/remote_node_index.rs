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

    #[cfg(test)]
    mod tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[derive(Deserialize)]
        struct TestSubject {
            #[serde(deserialize_with = "deserialize")]
            lts: Option<String>,
        }

        #[test]
        fn test_false_deserialization() {
            let json = serde_json::json!({ "lts": false });
            let subject: TestSubject = serde_json::from_value(json).expect("Can't deserialize json");
            assert_eq!(subject.lts, None);
        }

        #[test]
        fn test_value_deserialization() {
            let json = serde_json::json!({ "lts": "dubnium" });
            let subject: TestSubject = serde_json::from_value(json).expect("Can't deserialize json");
            assert_eq!(subject.lts, Some("dubnium".into()));
        }
    }
}

#[derive(Deserialize, Debug, Eq, Ord)]
pub struct IndexedNodeVersion {
    pub version: Version,
    #[serde(with = "lts_status")]
    pub lts: Option<String>,
    pub date: chrono::NaiveDate,
    pub files: Vec<String>,
}

impl PartialEq for IndexedNodeVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version.eq(&other.version)
    }
}

impl PartialOrd for IndexedNodeVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl IndexedNodeVersion {
    // pub fn download_url(&self, base_url: &reqwest::Url) -> reqwest::Url {
    //     use crate::system_info::{HostArch, HostPlatform};
    //     base_url
    //         .join(&format!("{}/", self.version))
    //         .unwrap()
    //         .join(&format!(
    //             "node-{node_ver}-{platform}-{arch}.tar.xz",
    //             node_ver = &self.version,
    //             platform = HostPlatform::default(),
    //             arch = HostArch::default(),
    //         ))
    //         .unwrap()
    // }
}

/// Prints
///
/// ```rust
/// use crate::remote_node_index::list;
/// ```
pub fn list(base_url: &reqwest::Url) -> Result<Vec<IndexedNodeVersion>, reqwest::Error> {
    let index_json_url = format!("{}/index.json", base_url);
    let value: Vec<IndexedNodeVersion> = reqwest::get(&index_json_url)?.json()?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_list() {
        let base_url = reqwest::Url::parse("https://nodejs.org/dist").unwrap();
        let expected_version = Version::parse("12.0.0").unwrap();
        let mut versions = list(&base_url).expect("Can't get HTTP data");
        assert_eq!(
            versions
                .drain(..)
                .find(|x| x.version == expected_version)
                .map(|x| x.version),
            Some(expected_version)
        );
    }
}

// pub fn print_stuff() -> Result<(), reqwest::Error> {
//     let value: Vec<IndexedNodeVersion> =
//         reqwest::get("https://nodejs.org/dist/index.json")?.json()?;

//     let xxx: Vec<_> = value.iter().filter(|x| x.lts.is_some()).collect();
//     let url = reqwest::Url::parse("https://nodejs.org/dist/").unwrap();

//     let url = xxx.first().unwrap().download_url(&url);
//     dbg!(&url);

//     println!("a");
//     let file_response = reqwest::get(url)?;
//     let mut tar_stream = uncompressed_archive(file_response);
//     println!("got tar stream");
//     tar_stream.unpack("/tmp/").ok();
//     println!("unpacked!");

//     Ok(())
// }
