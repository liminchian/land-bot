use reqwest::{self, blocking::Client};
use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug)]
pub struct MoiQaData {
    pub question: String,
    pub answers: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.land.moi.gov.tw/chhtml/landQA/55";
    let client = Client::new();
    let mut data = vec![];

    for page in 1..100 {
        let response = client
            .get(url)
            .query(&[("qphclass", ""), ("pagenum", &page.to_string())])
            .send()?;
        let body = response.text()?;
        let document = Html::parse_document(&body);
        let question_selector = Selector::parse("p.qa_question")?;
        let answer_selector = Selector::parse("div.qa_text")?;
        let questions = document.select(&question_selector);
        let answers = document.select(&answer_selector);
        let page_data: Vec<MoiQaData> = questions
            .zip(answers)
            .map(|(q, a)| MoiQaData {
                question: q.text().collect::<Vec<_>>().join(" ").to_string(),
                answers: a.text().collect::<Vec<_>>().join(" ").to_string(),
            })
            .collect();
        if !page_data.is_empty() {
            println!(
                "完成第 {} 頁的爬取，共取得 {} 筆問答資料",
                page,
                page_data.len()
            );
            data.extend(page_data);
        } else {
            println!("最終頁為 {}", page - 1);
            break;
        }
    }

    println!("{:?}", data);

    Ok(())
}
