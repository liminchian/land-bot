use clap::{ArgMatches, Command};

use crate::settings::Settings;

mod migrate;
mod moi;

pub fn configuration(command: Command) -> Command {
    command
        .subcommand(moi::configure())
        .subcommand(migrate::configure())
}

pub fn handler(matches: &ArgMatches, settings: Settings) -> anyhow::Result<()> {
    moi::handle(matches)?;
    migrate::handle(matches, settings)?;

    Ok(())
}
