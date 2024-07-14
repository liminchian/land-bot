use anyhow::Result;
use api::{commands, settings};
use clap::{Arg, Command};
use dotenv::dotenv;
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, Registry};

fn main() -> Result<()> {
    dotenv().ok();
    let module_root = std::fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    let pkg_root = module_root.parent().unwrap().parent().unwrap();

    let mut command = Command::new("api")
        .version("1.0")
        .author("liminchian <liminchian@gmail.com>")
        .about("提供後台資料的RESTful API")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("PATH")
                .help("設定檔案路徑")
                .default_value("config.json"),
        );

    command = commands::configuration(command);

    let matches = command.get_matches();

    let config_location = matches.get_one::<String>("config").unwrap();
    let config_location = pkg_root.join(config_location);
    let config_location = config_location.to_str().unwrap();

    let settings = settings::Settings::new(config_location, "API")?;

    let subscriber = Registry::default()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default().with_writer(std::io::stdout));

    tracing::subscriber::set_global_default(subscriber).expect("設置日誌時出現錯誤");

    commands::handler(&matches, &settings)?;

    Ok(())
}
