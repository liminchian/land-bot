use anyhow::{Error as E, Result};
use clap::{ArgMatches, Command};
use reqwest::{self, blocking::Client};
use scraper::{Html, Selector};
use uuid::Uuid;

use crate::settings::Settings;

static MOI_FAQ_URL: &str = "https://www.land.moi.gov.tw/chhtml/landQA/55";

#[derive(Debug)]
pub struct MoiQaData {
    pub id: String,
    pub question: String,
    pub answers: String,
}

pub fn configure() -> Command {
    Command::new("moi").about("台灣內政部地政相關常見問答資料爬取")
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("moi") {
        crawl()?;
    }
    Ok(())
}

pub fn crawl() -> Result<()> {
    let client = Client::new();
    let mut data = vec![];

    for page in 1..100 {
        let response = client
            .get(MOI_FAQ_URL)
            .query(&[("qphclass", ""), ("pagenum", &page.to_string())])
            .send()?;
        let body = response.text()?;
        let document = Html::parse_document(&body);

        let question_selector =
            Selector::parse("p.qa_question").map_err(|op| E::msg(op.to_string()))?;
        let answer_selector =
            Selector::parse("div.qa_text").map_err(|op| E::msg(op.to_string()))?;

        let questions = document.select(&question_selector);
        let answers = document.select(&answer_selector);

        let page_data: Vec<MoiQaData> = questions
            .zip(answers)
            .map(|(q, a)| MoiQaData {
                id: Uuid::new_v4().to_string(),
                question: q.text().collect::<Vec<_>>().join(" ").to_string(),
                answers: a.text().collect::<Vec<_>>().join(" ").to_string(),
            })
            .collect();
        if !page_data.is_empty() {
            info!(
                "完成第 {} 頁的爬取，共取得 {} 筆問答資料",
                page,
                page_data.len()
            );
            data.extend(page_data);
        } else {
            info!("最終頁為 {}", page - 1);
            break;
        }
    }

    Ok(())
}

pub async fn add(data: MoiQaData) -> Result<()> {
    todo!("寫入資料進入資料庫中")
}

pub async fn update(id: String) -> Result<()> {
    todo!("更新資料庫中的資料")
}

pub async fn delete(id: String) -> Result<MoiQaData> {
    todo!("刪除資料庫中的資料")
}
