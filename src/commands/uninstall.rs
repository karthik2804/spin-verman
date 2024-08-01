use crate::constants::{ROOT_FOLDER, VERSIONS_DIRECTORY};
use crate::utils::get_home_directory;
use crate::versions::Version;
use anyhow::{bail, Result};
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
pub struct UninstallCommand {
    version: Version,
}

impl UninstallCommand {
    pub async fn run(self) -> Result<()> {
        let home_dir = get_home_directory()?;
        let version_dir = home_dir
            .join(ROOT_FOLDER)
            .join(VERSIONS_DIRECTORY)
            .join(&self.version.to_string());

        println!("{version_dir:#?}");

        if version_dir.exists() {
            fs::remove_dir_all(&version_dir)?;
            println!("Uninstalled version '{}'", self.version.to_string());
        } else {
            bail!("Version '{}' not installed", self.version.to_string());
        }

        Ok(())
    }
}
