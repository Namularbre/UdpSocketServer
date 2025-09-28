use crate::config::Config;

mod config;
mod utils;
mod server;

fn main() {
    let config = Config::from_json("config.json".to_string());
    server::listen(&config);
}
