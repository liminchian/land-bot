use clap::{Arg, ArgMatches, Command};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::{path, settings::Settings, state::ApplicationState};

pub fn configure() -> Command {
    Command::new("serve").about("啟用 api 伺服器").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .value_parser(clap::value_parser!(u16))
            .help("預設的伺服器接口")
            .default_value("8080"),
    )
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("serve") {
        let port = *matches.get_one::<u16>("port").unwrap();

        start_tokio(port, _settings)?;
    }

    Ok(())
}

fn start_tokio(port: u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let state = Arc::new(ApplicationState::new(settings)?);
            let addr = format!("0.0.0.0:{}", port);
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            let app = path::configure(state).layer(TraceLayer::new_for_http());

            info!("啟動 Axum 伺服器在接口 {}", port);

            axum::serve(listener, app).await.unwrap();

            Ok::<(), anyhow::Error>(())
        })?;

    std::process::exit(0);
}
