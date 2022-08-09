use crate::handlers::*;
use matrix_sdk::{config::SyncSettings, Client};
use url::Url;

pub async fn login(homeserver_url: String, username: &str, password: &str) -> Result<(), matrix_sdk::Error> {
    let homeserver_url = Url::parse(&homeserver_url).expect("Couldn't parse the homeserver URL");
    let client = Client::new(homeserver_url).await.unwrap();

    client.register_event_handler(on_room_message).await;
    client.register_event_handler(on_stripped_state_member).await;

    client.login(username, password, None, Some("rust-sdk")).await?;
    client.sync(SyncSettings::new()).await;

    Ok(())
}
