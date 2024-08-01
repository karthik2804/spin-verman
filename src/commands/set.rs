use crate::constants::VERSIONS_DIRECTORY;
use crate::utils::{get_home_directory, list_installed_versions};
use crate::versions::Version;
use anyhow::Result;
use clap::Parser;
use std::fs;
use std::os::unix;
use std::os::unix::fs::PermissionsExt;

#[derive(Parser, Debug)]
pub struct SetCommand {
    version: Version,
}

impl SetCommand {
    pub async fn run(self) -> Result<()> {
        let installed_versions = list_installed_versions()?;

        if installed_versions.contains(&self.version.to_string()) {
            let home_dir = get_home_directory()?;
            let spin_verman_dir = home_dir.join(".spin-verman");
            let version_dir = spin_verman_dir
                .join(VERSIONS_DIRECTORY)
                .join(&self.version.to_string());
            let spin_executable_dest = spin_verman_dir.join("spin");
            let spin_executable_src = version_dir.join("spin");

            println!("{spin_executable_src:#?}");
            println!("{spin_executable_dest:#?}");
            // Create the spin executable if it doesn't exist
            if !spin_executable_src.exists() {
                fs::create_dir_all(&spin_verman_dir)?;
                fs::write(&spin_executable_src, "")?;
                let mut permissions = fs::metadata(&spin_executable_src)?.permissions();
                permissions.set_mode(0o755); // Set execute permissions
                fs::set_permissions(&spin_executable_src, permissions)?;
            }

            // Remove existing symlink if exists
            if spin_executable_dest.exists() {
                fs::remove_file(&spin_executable_dest)?;
            }

            // Create the symbolic link
            unix::fs::symlink(&spin_executable_src, &spin_executable_dest)?;
            println!("Set version '{}' successfully", self.version);
        } else {
            println!("Version '{}' is not installed", self.version);
        }

        Ok(())
    }
}
