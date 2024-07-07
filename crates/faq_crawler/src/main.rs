use anyhow::Result;
use clap::{Arg, Command};
use dotenv::dotenv;
use faq_crawler::commands;

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

    commands::handler(&matches)?;

    Ok(())
}
