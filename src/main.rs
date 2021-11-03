mod tidy;

use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use clap::Parser;

use tidy::Engine;

const VERSION: &str = env!("CARGO_PKG_VERSION");

async fn run(config: String) -> Result<()> {
    let config_path: PathBuf = shellexpand::tilde(&config).to_string().into();

    let raw_data = fs::read_to_string(config_path)?;

    let engine = Engine::start(&raw_data)?;

    tokio::signal::ctrl_c().await?;

    engine.stop().await
}

#[derive(Parser)]
enum Action {
    #[clap(name = "run")]
    Run,
}

#[derive(Parser)]
#[clap(version = VERSION, author = "Purposed")]
struct Options {
    #[clap(
        long = "cfg",
        short = 'c',
        default_value = "~/.config/purposed/tidy/config.json"
    )]
    config: String,

    #[clap(subcommand)]
    action: Action,
}

impl Options {
    pub async fn execute(self) -> Result<()> {
        match self.action {
            Action::Run => run(self.config).await?,
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    if let Err(e) = Options::parse().execute().await {
        log::error!("{}", e);
    }
}
