use anyhow::Result;
use clap::{Arg, Command};
use crawler::{commands, settings};
use dotenv::dotenv;

fn main() -> Result<()> {
    dotenv().ok();

    let mut command = Command::new("faq_crawler")
        .version("1.0")
        .author("liminchian <liminchian@gmail.com>")
        .about("爬取地政相關常見問題")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("設定檔案路徑")
                .default_value("config.json"),
        );

    command = commands::configuration(command);

    let matches = command.get_matches();

    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");

    let settings = settings::Settings::new(config_location, "CRAWLER")?;

    commands::handler(&matches, &settings)?;

    Ok(())
}
