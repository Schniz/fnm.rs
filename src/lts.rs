use crate::remote_node_index::IndexedNodeVersion;
use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum LtsType {
    /// lts-*, lts/*
    Any,
    /// lts-erbium, lts/erbium
    CodeName(String),
}

impl From<&str> for LtsType {
    fn from(s: &str) -> Self {
        if s == "*" || s == "latest" {
            Self::Any
        } else {
            Self::CodeName(s.to_string())
        }
    }
}

impl Display for LtsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "latest"),
            Self::CodeName(s) => write!(f, "{}", s),
        }
    }
}

impl LtsType {
    pub fn pick_latest<'vec>(
        &self,
        versions: &'vec Vec<IndexedNodeVersion>,
    ) -> Option<&'vec IndexedNodeVersion> {
        match self {
            Self::Any => versions.iter().filter(|x| x.lts.is_some()).last(),
            Self::CodeName(s) => versions
                .iter()
                .filter(|x| match &x.lts {
                    None => false,
                    Some(x) => s.to_lowercase() == x.to_lowercase(),
                })
                .last(),
        }
    }
}
