use clap::{ArgMatches, Command};

use crate::settings::Settings;

mod crawl;
mod hello;
mod migrate;
mod serve;

pub fn configuration(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
        .subcommand(migrate::configure())
        .subcommand(crawl::configure())
}

pub fn handler(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches, settings)?;
    serve::handle(matches, settings)?;
    migrate::handle(matches, settings)?;
    crawl::handle(matches, settings)?;

    Ok(())
}
