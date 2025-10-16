mod server;
mod connection;
mod broker;
mod storage;
mod message;
mod cli;
mod utils;

fn main() {
    println!("Starting HyperBus...");
    server::start();
}
