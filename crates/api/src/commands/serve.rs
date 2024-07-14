use axum::Router;
use clap::{Arg, ArgMatches, Command};

use crate::settings::Settings;

pub fn configure() -> Command {
    Command::new("serve").about("啟用 api 伺服器").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .help("預設的伺服器接口")
            .default_value("8080"),
    )
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("serve") {
        let port = *matches.get_one::<u16>("port").unwrap();

        start_tokio(port, _settings)?;
    }

    Ok(())
}

fn start_tokio(port: u16, _settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let addr = format!("0.0.0.0:{}", port);
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            let app = Router::new();

            axum::serve(listener, app).await.unwrap();

            Ok::<(), anyhow::Error>(())
        })?;

    std::process::exit(0);
}
