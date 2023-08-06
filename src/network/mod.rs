use anyhow::Result;
use tracing::info;

use crate::config::Config;

pub async fn p2p_server(_config: Config) -> Result<()> {
    info!("p2p server up");
    Ok(())
}
