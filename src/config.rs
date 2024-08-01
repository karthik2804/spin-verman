use std::fs;

use crate::{
    constants::{CONFIG_FIILENAME, ROOT_FOLDER},
    utils,
};
use anyhow::{anyhow, Result};

pub fn init_config() -> Result<()> {
    let home_dir = utils::get_home_directory()?;
    let spin_verman_dir = home_dir.join(ROOT_FOLDER);
    let config_file = spin_verman_dir.join(CONFIG_FIILENAME);

    if !spin_verman_dir.exists() {
        generate_default_directory()?;
    }

    if !config_file.exists() {
        generate_default_config()?;
    }

    Ok(())
}

fn generate_default_directory() -> Result<()> {
    println!("Generating default structure");
    let home_dir = utils::get_home_directory()?;
    let spin_verman_dir = home_dir.join(ROOT_FOLDER);
    fs::create_dir(&spin_verman_dir)
        .map_err(|err| anyhow!("Failed to create directory: {}", err))?;

    Ok(())
}

fn generate_default_config() -> Result<()> {
    Ok(())
}
