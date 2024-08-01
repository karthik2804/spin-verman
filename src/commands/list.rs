use anyhow::Result;
use clap::Parser;

use crate::utils::list_installed_versions;
#[derive(Parser, Debug)]
pub struct ListCommand {}

impl ListCommand {
    pub async fn run(self) -> Result<()> {
        let installed_versions = list_installed_versions()?;
        for version in installed_versions {
            println!("{}", version);
        }
        Ok(())
    }
}
