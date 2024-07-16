use clap::{ArgMatches, Command};
use migration::{sea_orm::Database, Migrator, MigratorTrait};

use crate::settings::Settings;

pub fn configure() -> Command {
    Command::new("migrate").about("Run database migrations")
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("migrate") {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let db_url = settings.database.url.clone().unwrap_or("".to_owned());
                let conn = Database::connect(db_url).await.expect("資料庫連線失敗");

                Migrator::up(&conn, None).await.unwrap();
            })
    }
    Ok(())
}
