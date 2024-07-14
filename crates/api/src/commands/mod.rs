use clap::{ArgMatches, Command};

use crate::settings::Settings;

mod hello;
mod migrate;
mod serve;

pub fn configuration(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
}

pub fn handler(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches, _settings)?;
    serve::handle(matches, _settings)?;

    Ok(())
}
