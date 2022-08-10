mod config;
mod handlers;
mod util;

#[macro_use]
extern crate colour;

use crate::{config::BotConfig, util::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let BotConfig {
        homeserver_url,
        username,
        password,
        sled_path,
    } = BotConfig::load().expect("Error loading config file!");

    login_and_sync(homeserver_url, username, password, sled_path).await
}
