use crate::config::Config;

pub fn start(config: Config) {
    println!("Server started on {}:{}", config.host, config.port);
}
