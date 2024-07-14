use clap::{ArgMatches, Command};

use crate::settings::Settings;

pub fn configure() -> Command {
    Command::new("hello").about("打印出 Hello World")
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("hello") {
        println!("Hello World");
    }

    Ok(())
}
