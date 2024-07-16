use std::str::FromStr;

use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

use crate::settings::Settings;
use crawler::{Crawler, CrawlerResult, CrawlerTrait};
use entity::qa::ActiveModel;
use sea_orm::entity::Set;
use sea_orm::{ActiveModelTrait, Database};

pub fn configure() -> Command {
    Command::new("crawl").about("問答資料的爬蟲").arg(
        Arg::new("source")
            .required(true)
            .value_name("SOURCE")
            .help("爬蟲的網站類型"),
    )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches("crawler") {
        let source = matches.get_one::<String>("source").unwrap();
        let mut crawler = Crawler::from_str(&source)?.into();
        let data = crawler.crawl()?;
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let db_url = settings.database.url.clone().unwrap_or("".to_string());
                let conn = Database::connect(db_url)
                    .await
                    .expect("資料庫連線時發生錯誤");
                for result in data {
                    let model = ActiveModel {
                        id: Set(result.get("id").to_string()),
                        source: Set(source.to_string()),
                        question: Set(result.get("question").to_string()),
                        answer: Set(result.get("answer").to_string()),
                    };
                    if let Ok(_model) = model.save(&conn).await {
                        info!("{} 資料加入成功", &source);
                    } else {
                        info!("{} 資料加入失敗", &source);
                    };
                }
                Ok::<(), anyhow::Error>(())
            })?;
    }
    Ok(())
}
