#[macro_use]
extern crate tracing;
pub mod cli;
mod moi;

use std::str::FromStr;

use moi::MoiCrawler;

pub trait CrawlerTrait {
    fn crawl(&mut self) -> anyhow::Result<()>;
    fn source(&self) -> String;
}

#[derive(Debug)]
pub enum Crawler {
    Moi,
}

impl FromStr for Crawler {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "moi" => Ok(Crawler::Moi),
            _ => Err(anyhow::Error::msg("不在清單中的來源")),
        }
    }
}

impl Crawler {
    pub fn into(self) -> impl CrawlerTrait {
        match self {
            Crawler::Moi => MoiCrawler::default(),
            _ => todo!(),
        }
    }
}
