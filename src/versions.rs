use anyhow::anyhow;
use semver::Version as SemVer;
use std::{fmt, str::FromStr};

#[derive(Debug)]
pub enum Version {
    Canary,
    Release(String),
}

impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "canary" => Ok(Version::Canary),
            v if SemVer::parse(v).is_ok() => Ok(Version::Release(v.to_string())),
            _ => Err(anyhow!("Invalid version format: {}", s)),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Version::Canary => write!(f, "canary"),
            Version::Release(v) => write!(f, "v{}", v),
        }
    }
}
