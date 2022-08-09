mod config;
mod handlers;
mod util;

#[macro_use]
extern crate colour;

use crate::{config::BotConfig, util::*};

#[tokio::main]
async fn main() -> Result<(), matrix_sdk::Error> {
    let BotConfig {
        homeserver_url,
        username,
        password,
    } = BotConfig::load().expect("Error loading config file!");

    login(homeserver_url, &username, &password).await
}
