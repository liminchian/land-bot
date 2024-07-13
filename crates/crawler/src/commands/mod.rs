use clap::{ArgMatches, Command};

use crate::settings::Settings;

mod hello;
mod migrate;

pub fn configuration(command: Command) -> Command {
    command.subcommand(hello::configure())
}

pub fn handler(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches, _settings)?;
    Ok(())
}
