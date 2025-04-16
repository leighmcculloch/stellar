mod horizon;
mod models;
mod rpc;

use std::net::SocketAddr;

use clap::Parser;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use log::info;
use rpc::{StellarRpcApiServer, StellarRpcServer};

/// A JSON-RPC server that uses Stellar Horizon API as a data source
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The address to bind the JSON-RPC server to
    #[clap(short, long, default_value = "127.0.0.1:8545")]
    bind_address: SocketAddr,

    /// The Horizon API server URL
    #[clap(long, default_value = "https://horizon-testnet.stellar.org")]
    horizon_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();
    let horizon_url = args.horizon_url;
    let bind_address = args.bind_address;

    // Create the RPC server with the Horizon client
    let rpc = StellarRpcServer::new(horizon_url.clone())?;

    // Build the JSON-RPC server
    let server = ServerBuilder::default().build(bind_address).await?;

    // Register the RPC API methods
    let server_handle = server.start(rpc.into_rpc());

    info!("JSON-RPC server started at {}", bind_address);
    info!("Using Horizon API at {}", horizon_url);

    // Keep the server running until terminated
    wait_for_shutdown(server_handle).await?;

    Ok(())
}

async fn wait_for_shutdown(server_handle: ServerHandle) -> anyhow::Result<()> {
    let ctrl_c = tokio::signal::ctrl_c();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl-C, shutting down...");
        }
    }

    server_handle.stop()?;
    info!("Server stopped");
    Ok(())
}

