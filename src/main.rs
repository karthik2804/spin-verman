use anyhow::{Error, Result};
use clap::{FromArgMatches, Parser};
mod commands;
mod config;
mod constants;
mod utils;
mod versions;

use commands::{
    install::InstallCommand, list::ListCommand, set::SetCommand, uninstall::UninstallCommand,
};
use config::init_config;

/// Returns build information, similar to: 0.1.0 (2be4034 2022-03-31).
const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    " ",
    env!("VERGEN_GIT_COMMIT_DATE"),
    ")"
);

#[derive(Parser)]
#[clap(author, version = VERSION, about, long_about = None)]
#[clap(propagate_version = true)]
enum VermanCli {
    Set(SetCommand),
    Uninstall(UninstallCommand),
    Install(InstallCommand),
    List(ListCommand),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut app = VermanCli::clap();
    app.set_bin_name("spin verman");
    let matches = app.get_matches();
    let cli = VermanCli::from_arg_matches(&matches)?;

    init_config()?;

    match cli {
        VermanCli::List(cmd) => cmd.run().await,
        VermanCli::Set(cmd) => cmd.run().await,
        VermanCli::Uninstall(cmd) => cmd.run().await,
        VermanCli::Install(cmd) => cmd.run().await,
    }
}
