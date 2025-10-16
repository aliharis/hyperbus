use clap::Parser;

/// Configuration options for HyperBus
#[derive(Parser, Debug)]
#[command(name = "hyperbus")]
pub struct Config {
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    #[arg(long, default_value_t = 9000)]
    pub port: u16,

    #[arg(long, default_value = "./data")]
    pub data_dir: String,
}
