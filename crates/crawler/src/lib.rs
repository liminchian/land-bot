#[macro_use]
extern crate tracing;
mod moi;

use std::str::FromStr;

pub trait CrawlerTrait {
    fn crawl(&mut self) -> anyhow::Result<Vec<impl CrawlerResult>>;
    fn source(&self) -> String;
}

pub trait CrawlerResult {
    fn get(&self, name: &str) -> impl ToString;
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
            Crawler::Moi => moi::CrawlerImpl,
            _ => todo!(),
        }
    }
}
