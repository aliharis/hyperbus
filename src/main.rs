mod config;
mod server;
mod connection;
mod broker;
mod storage;
mod message;
mod cli;
mod utils;

use clap::Parser;
use config::Config;
use tracing_subscriber;
use tracing::{info, error, debug};

fn main() {
    // 1️⃣ Initialize the logger
    tracing_subscriber::fmt()
        .with_target(false) // don't show the module name
        .with_level(true)   // show log levels like INFO, ERROR
        .init();

    // 2️⃣ Parse the CLI config
    let config = Config::parse();

    // 3️⃣ Log the startup sequence
    info!("Starting HyperBus...");
    info!("Host: {}", config.host);
    info!("Port: {}", config.port);
    info!("Data directory: {}", config.data_dir);

    // 4️⃣ Start the server
    info!("Starting server...");
    server::start(config);
}
