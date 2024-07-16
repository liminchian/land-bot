#[macro_use]
extern crate tracing;
pub mod cli;
mod moi;

use moi::MoiCrawler;

pub trait CrawlerTrait {
    fn crawl(&mut self) -> anyhow::Result<()>;
    fn source(&self) -> String;
}

pub enum Crawler {
    Moi(MoiCrawler),
}

impl Crawler {
    pub fn into(self) -> impl CrawlerTrait {
        match self {
            Crawler::Moi(crawler) => crawler,
            _ => todo!(),
        }
    }
}
