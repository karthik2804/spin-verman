use crate::versions::Version;
use anyhow::{anyhow, Result};
use clap::Parser;
use flate2::read::GzDecoder;
use std::{fs, io};
use tar::Archive;

use crate::{
    constants::{ROOT_FOLDER, VERSIONS_DIRECTORY},
    utils::{current_platform, get_home_directory, Platform},
};

#[derive(Parser, Debug)]
pub struct InstallCommand {
    #[clap(long, short)]
    update: bool,
    version: Version,
}

impl InstallCommand {
    pub async fn run(self) -> Result<()> {
        let version = self.version;
        let platform = current_platform()?;
        download_spin(version.to_string(), &platform, self.update).await?;
        Ok(())
    }
}

async fn download_spin(version: String, platform: &Platform, update: bool) -> Result<()> {
    let home_dir = get_home_directory()?;
    let spin_ver_dir = home_dir
        .join(ROOT_FOLDER)
        .join(VERSIONS_DIRECTORY)
        .join(version.clone());

    if spin_ver_dir.exists() {
        println!("version '{}' already installed...", version);
        if !update {
            println!("Skipped");
            return Ok(());
        }
        println!("updating exisitng installation");
        fs::remove_dir_all(&spin_ver_dir)?;
    }

    let file = format!("spin-{}-{}-{}.tar.gz", version, platform.os, platform.arch);
    let url = format!(
        "https://github.com/fermyon/spin/releases/download/{}/{}",
        version, file
    );
    let response = reqwest::get(&url).await?;
    if !response.status().is_success() {
        return Err(anyhow!("Failed to download Spin: {}", response.status()));
    }

    fs::create_dir_all(&spin_ver_dir)?;

    let file_path = spin_ver_dir.join(&file);
    let mut file = fs::File::create(&file_path)?;

    io::copy(&mut response.bytes().await?.as_ref(), &mut file)?;

    // Decompress the file
    let file = fs::File::open(&file_path)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(&spin_ver_dir)?;

    // Remove the compressed file
    fs::remove_file(&file_path)?;
    println!("Installed version {version}");
    Ok(())
}
