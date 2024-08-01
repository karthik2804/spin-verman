use anyhow::{anyhow, bail, Result};
use std::{fmt, fs, path::PathBuf};

use crate::constants::{ROOT_FOLDER, VERSIONS_DIRECTORY};

pub fn get_home_directory() -> Result<PathBuf> {
    match dirs::home_dir() {
        Some(path) => Ok(path),
        None => bail!("unable to access HOME directory"),
    }
}

#[derive(Debug)]
pub enum Architecture {
    Amd64,
    Aarch64,
}

#[derive(Debug)]
pub enum OperatingSystem {
    Linux,
    MacOS,
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperatingSystem::Linux => write!(f, "linux"),
            OperatingSystem::MacOS => write!(f, "macos"),
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::Amd64 => write!(f, "amd64"),
            Architecture::Aarch64 => write!(f, "aarch64"),
        }
    }
}

#[derive(Debug)]
pub struct Platform {
    pub os: OperatingSystem,
    pub arch: Architecture,
}

pub fn current_platform() -> Result<Platform> {
    let arch = detect_architecture()?;

    let os_type = std::env::consts::OS;
    let os = match os_type {
        "linux" => OperatingSystem::Linux,
        "macos" => OperatingSystem::MacOS,
        _ => {
            return Err(anyhow!(
                "The OS type {} is not supported by this script.",
                os_type
            ))
        }
    };

    Ok(Platform { os, arch })
}

fn detect_architecture() -> Result<Architecture, anyhow::Error> {
    if cfg!(target_arch = "x86_64") {
        Ok(Architecture::Amd64)
    } else if cfg!(target_arch = "aarch64") {
        Ok(Architecture::Aarch64)
    } else {
        Err(anyhow::anyhow!("Unsupported architecture"))
    }
}

pub fn list_installed_versions() -> Result<Vec<String>> {
    let home_dir = get_home_directory()?;
    let spin_verman_dir = home_dir.join(ROOT_FOLDER).join(VERSIONS_DIRECTORY);
    let mut installed_versions = Vec::new();

    if spin_verman_dir.exists() {
        for entry in fs::read_dir(spin_verman_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(version) = entry.file_name().to_str() {
                    installed_versions.push(version.to_string());
                }
            }
        }
    }

    Ok(installed_versions)
}
