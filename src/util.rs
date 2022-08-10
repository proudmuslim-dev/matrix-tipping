use crate::handlers::*;
use std::path::PathBuf;
//use matrix_sdk::{config::SyncSettings, Client};

use matrix_sdk::{config::SyncSettings, Client};

pub async fn login_and_sync(
    homeserver_url: String,
    username: String,
    password: String,
    sled_path: Option<String>,
) -> anyhow::Result<()> {
    #[allow(unused_mut)]
    let mut client_builder = Client::builder().homeserver_url(homeserver_url);

    {
        // The location to save files to
        let mut path;

        if let Some(s) = sled_path {
            path = PathBuf::from(s);
        } else {
            path = dirs::home_dir().expect("no home directory found");
            path.push("matrix-tipping");
        }

        let state_store = matrix_sdk_sled::StateStore::open_with_path(path)?;

        client_builder = client_builder.state_store(Box::new(state_store));
    }

    let client = client_builder.build().await.unwrap();
    client.login(&username, &password, None, Some("command bot")).await?;

    println!("Logged in as {username}");

    // An initial sync to set up state and so our bot doesn't respond to old
    // messages. If the `StateStore` finds saved state in the location given the
    // initial sync will be skipped in favor of loading state from the store.
    client.sync_once(SyncSettings::default()).await.unwrap();
    client.register_event_handler(on_room_message).await;
    client.register_event_handler(on_stripped_state_member).await;

    let settings = SyncSettings::default().token(client.sync_token().await.unwrap());
    client.sync(settings).await;

    Ok(())
}
