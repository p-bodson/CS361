use clap::Parser;
use std::time::Duration;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(short = 'd', long, default_value = "../data/db.json")]
    pub database: String,
    
    #[clap(short = 't', long, default_value = "200")]
    pub tick_rate: u64
}

#[derive(Debug)]
pub struct Config {
    pub database: PathBuf,
    pub tick_rate: Duration,
}

impl Config {
    pub fn tick_rate(mut self, rate: u64) -> Config
    {
        self.tick_rate = Duration::from_millis(rate);
        self
    }

    pub fn database<T>(mut self, path: T) -> Config
    where T: Into<PathBuf>
    {
        self.database = PathBuf::from(path.into());
        self
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            database: PathBuf::new(),
            tick_rate: Duration::new(0, 0),
        }
    }
}
