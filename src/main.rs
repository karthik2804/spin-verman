use clap::Parser;
mod commands;
use commands::list::ListCommand;

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
    Configure,
    Delete,
    Install,
    List(ListCommand),
}

fn main() -> Result<(), Error> {
    let mut app = VermanCli::clap();
    app.set_bin_name("spin cloud");
    let matches = app.get_matches();
    let cli = VermanCli::from_arg_matches(&matches)?;

    match cli {
        VermanCli::List(cmd) => cmd.run().await?,
        VermanCli::Configure => todo!(),
        VermanCli::Delete => todo!(),
        VermanCli::Install => todo!(),
    }
}
