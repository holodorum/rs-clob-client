//! Minimal reproduction for the initial orderbook dump bug.
//!
//! Before the fix, the initial snapshot (no `event_type` field) was silently
//! dropped, so `stream.next().await` would hang indefinitely.
//!
//! ```sh
//! RUST_LOG=debug,hyper_util=off,hyper=off,reqwest=off,h2=off,rustls=off \
//!   cargo run --example websocket_initial_dump --features clob,ws,tracing
//! ```

use std::str::FromStr as _;

use futures::StreamExt as _;
use polymarket_client_sdk::clob::ws::Client;
use polymarket_client_sdk::types::U256;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    tracing_subscriber::fmt::init();

    // Use the same asset IDs as the orderbook example
    let token = U256::from_str(
        "92703761682322480664976766247614127878023988651992837287050266308961660624165",
    )?;

    let client = Client::default();
    let mut stream = Box::pin(client.subscribe_orderbook(vec![token])?);

    info!("Waiting for initial orderbook snapshot...");

    // Before the fix this would hang forever — the initial dump was dropped.
    let book = stream.next().await;
    println!("{book:#?}");

    Ok(())
}