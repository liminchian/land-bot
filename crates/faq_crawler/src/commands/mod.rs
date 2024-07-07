use clap::{ArgMatches, Command};

mod moi;

pub fn configuration(command: Command) -> Command {
    command.subcommand(moi::configure())
}

pub fn handler(matches: &ArgMatches) -> anyhow::Result<()> {
    moi::handle(matches)?;

    Ok(())
}
