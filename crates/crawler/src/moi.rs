use anyhow::{Error as E, Result};
use reqwest::{self, blocking::Client};
use scraper::{Html, Selector};
use uuid::Uuid;

use crate::CrawlerTrait;

static ENTRY_URL: &str = "https://www.land.moi.gov.tw/chhtml/landQA/55";

#[derive(Debug, Clone)]
pub struct MoiData {
    pub id: String,
    pub question: String,
    pub answer: String,
}

pub struct MoiCrawler {
    pub data: Vec<MoiData>,
    pub source: String,
}

impl Default for MoiCrawler {
    fn default() -> Self {
        Self {
            data: vec![],
            source: "moi".to_string(),
        }
    }
}

impl CrawlerTrait for MoiCrawler {
    fn crawl(&mut self) -> Result<()> {
        let client = Client::new();

        for page in 1..100 {
            let response = client
                .get(ENTRY_URL)
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

            let page_data: Vec<MoiData> = questions
                .zip(answers)
                .map(|(q, a)| MoiData {
                    id: Uuid::new_v4().to_string(),
                    question: q.text().collect::<Vec<_>>().join(" ").to_string(),
                    answer: a.text().collect::<Vec<_>>().join(" ").to_string(),
                })
                .collect();
            if !page_data.is_empty() {
                info!(
                    "完成第 {} 頁的爬取，共取得 {} 筆問答資料",
                    page,
                    page_data.len()
                );
                self.data.extend(page_data);
            } else {
                info!("最終頁為 {}", page - 1);
                break;
            }
        }
        Ok(())
    }

    fn source(&self) -> String {
        self.source.to_string()
    }
}
