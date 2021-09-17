mod pb;

use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

struct ServerState {
    store: DashMap<String, Vec<u8>>,
}

impl ServerState {
    fn new() -> ServerState {   // Self
        ServerState { store: DashMap::new() }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self { store: DashMap::new() }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("hello tracing!");

    let state = Arc::new(ServerState::new());
    let addr = "0.0.0.0:8188";
    let listener = TcpListener::bind(addr).await?;

    info!("listening to {:?}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("new client: {:?} accepted", addr);
    }

    Ok(())
}
