use anyhow::Result;
use futures::prelude::*;
use libp2p::core::upgrade::Version;
use libp2p::{
    identity, noise, ping,
    swarm::{keep_alive, NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, Transport,
};
use std::net::TcpListener;
use tracing::info;

use crate::config::Config;

pub async fn p2p_server(config: Config) -> Result<()> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    info!("Local peer id: {local_peer_id:?}");

    let transport = tcp::tokio::Transport::default()
        .upgrade(Version::V1Lazy)
        .authenticate(noise::Config::new(&local_key)?)
        .multiplex(yamux::Config::default())
        .boxed();

    let mut swarm =
        SwarmBuilder::with_tokio_executor(transport, Behaviour::default(), local_peer_id).build();

    let port = get_port(config.default_p2p_port);
    let listen_addr = format!("/ip4/0.0.0.0/tcp/{}", port);
    swarm.listen_on(listen_addr.parse()?)?;

    // attempt to dial default port for local
    let default_listen_addr = format!("/ip4/127.0.0.1/tcp/{}", config.default_p2p_port);
    let remote: Multiaddr = default_listen_addr.parse()?;
    // don't dial yourself
    if !default_listen_addr.eq(&listen_addr) {
        swarm.dial(remote)?;
        info!("Dialed {default_listen_addr}");
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => info!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => info!("{event:?}"),
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => info!("connection established {endpoint:?} {peer_id:?}"),
            _ => {}
        }
    }
}

// walks up port numbers from default until one is available
fn get_port(default_port: u16) -> u16 {
    match TcpListener::bind(("0.0.0.0", default_port)) {
        Ok(_) => default_port,
        Err(_) => get_port(default_port + 1),
    }
}

/// Our network behaviour.
///
/// For illustrative purposes, this includes the [`KeepAlive`](keep_alive::Behaviour) behaviour so a continuous sequence of
/// pings can be observed.
#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}
