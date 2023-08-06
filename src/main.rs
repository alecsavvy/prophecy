use anyhow::Result;
use network::p2p_server;
use tracing::error;

use crate::config::Config;

pub mod api;
pub mod command;
pub mod config;
pub mod event;
pub mod network;
pub mod query;

use api::web_server;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let web_config = Config::default();
    let p2p_config = Config::default();

    let web_server = tokio::spawn(async { web_server(web_config).await });
    let p2p_server = tokio::spawn(async { p2p_server(p2p_config).await });

    let res = tokio::try_join!(web_server, p2p_server);
    if let Err(e) = res {
        error!("process crashed {}", e);
    }
    Ok(())
}
