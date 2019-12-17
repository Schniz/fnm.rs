#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
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

pub enum UserVersion {
    OnlyMajor(u64),
    MajorMinor(u64, u64),
    Semver(Version),
}

impl UserVersion {
    pub fn matches(&self, other: &Version) -> bool {
        match self {
            Self::OnlyMajor(major) => *major == other.major,
            Self::MajorMinor(major, minor) => *major == other.major && *minor == other.minor,
            Self::Semver(version) => version == other,
        }
    }

    pub fn to_version<'a, T>(&self, available_versions: T) -> Option<&'a Version>
    where
        T: IntoIterator<Item = &'a Version>,
    {
        available_versions
            .into_iter()
            .filter(|x| self.matches(x))
            .max()
    }
}

fn next_of<'a, T: std::str::FromStr, It: Iterator<Item = &'a str>>(i: &mut It) -> Option<T> {
    let x = i.next()?;
    T::from_str(x).ok()
}

impl std::fmt::Display for UserVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Semver(x) => x.fmt(f),
            Self::OnlyMajor(major) => write!(f, "v{}.x.x", major),
            Self::MajorMinor(major, minor) => write!(f, "v{}.{}.x", major, minor),
        }
    }
}

impl std::fmt::Debug for UserVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserVersion({})", self)
    }
}

impl std::str::FromStr for UserVersion {
    type Err = semver::SemVerError;
    fn from_str(s: &str) -> Result<UserVersion, Self::Err> {
        match Version::parse(s) {
            Ok(v) => Ok(Self::Semver(v)),
            Err(e) => {
                let mut parts = s.trim().split('.');
                match (next_of::<u64, _>(&mut parts), next_of::<u64, _>(&mut parts)) {
                    (Some(major), None) => Ok(Self::OnlyMajor(major)),
                    (Some(major), Some(minor)) => Ok(Self::MajorMinor(major, minor)),
                    _ => Err(e),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_to_version() {
        let expected = Version::parse("6.1.0").unwrap();
        let versions = vec![
            Version::parse("6.0.0").unwrap(),
            Version::parse("6.0.1").unwrap(),
            expected.clone(),
            Version::parse("7.0.1").unwrap(),
        ];
        let result = UserVersion::OnlyMajor(6).to_version(&versions);

        assert_eq!(result, Some(&expected));
    }

    #[test]
    fn test_major_minor_to_version() {
        let expected = Version::parse("6.0.1").unwrap();
        let versions = vec![
            Version::parse("6.0.0").unwrap(),
            Version::parse("6.1.0").unwrap(),
            expected.clone(),
            Version::parse("7.0.1").unwrap(),
        ];
        let result = UserVersion::MajorMinor(6, 0).to_version(&versions);

        assert_eq!(result, Some(&expected));
    }

    #[test]
    fn test_semver_to_version() {
        let expected = Version::parse("6.0.0").unwrap();
        let versions = vec![
            expected.clone(),
            Version::parse("6.1.0").unwrap(),
            Version::parse("6.0.1").unwrap(),
            Version::parse("7.0.1").unwrap(),
        ];
        let result = UserVersion::Semver(expected.clone()).to_version(&versions);

        assert_eq!(result, Some(&expected));
    }
}
