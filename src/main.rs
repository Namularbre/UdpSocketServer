use crate::config::Config;

mod config;
mod vec_to_string;
mod server;

fn main() {
    let config = Config::from_json("config.json".to_string());
    server::listen(&config);
}
