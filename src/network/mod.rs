use anyhow::Result;
use tracing::info;

use crate::config::Config;

pub async fn p2p_server(_config: Config) -> Result<()> {
    loop {
        info!("p2p server up");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
