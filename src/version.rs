#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Version(semver::Version);

impl Version {
    pub fn parse<S: AsRef<str>>(version_str: S) -> Result<Self, semver::SemVerError> {
        let version_plain = version_str.as_ref().trim_start_matches('v');
        let sver = semver::Version::parse(&version_plain)?;
        Ok(Self(sver))
    }

    pub fn v_str(&self) -> String {
        format!("v{}", self.0)
    }
}

impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version_str = String::deserialize(deserializer)?;
        Version::parse(version_str).map_err(serde::de::Error::custom)
    }
}

impl std::ops::Deref for Version {
    type Target = semver::Version;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<semver::Version> for Version {
    fn as_ref(&self) -> &semver::Version {
        &self.0
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v")?;
        self.0.fmt(f)
    }
}

impl std::str::FromStr for Version {
    type Err = semver::SemVerError;
    fn from_str(s: &str) -> Result<Version, Self::Err> {
        Self::parse(s)
    }
}
