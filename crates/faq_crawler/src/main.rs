use reqwest;
use scraper::{Html, Selector};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.land.moi.gov.tw/chhtml/landQA/55";
    let response = reqwest::blocking::get(url)?;

    let body = response.text()?;
    let document = Html::parse_document(&body);

    let question_selector = Selector::parse("div.question")?;
    let answer_selector = Selector::parse("div.answer")?;

    let questions = document.select(&question_selector);
    let answers = document.select(&answer_selector);

    Ok(())
}
