use matrix_sdk::{
    room::Room,
    ruma::events::room::{
        member::StrippedRoomMemberEvent,
        message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent, TextMessageEventContent},
    },
    Client,
};

use tokio::time::{sleep, Duration};

// TODO: Use a different event to avoid running commands twice
pub async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    if let Room::Joined(room) = room {
        let msg_body = match event.content.msgtype {
            MessageType::Text(TextMessageEventContent { body, .. }) => body,
            _ => return,
        };

        if msg_body.starts_with("!ping") {
            let content = RoomMessageEventContent::text_plain("🏴");

            yellow_ln!("Sending...");

            room.send(content, None).await.unwrap();

            green_ln!("Message sent. 🏴");
        }
    }
}

pub async fn on_stripped_state_member(room_member: StrippedRoomMemberEvent, client: Client, room: Room) {
    if room_member.state_key != client.user_id().await.unwrap() {
        return;
    }

    if let Room::Invited(room) = room {
        yellow_ln!("Autojoining room {}", room.room_id());
        let mut delay = 2;

        while let Err(err) = room.accept_invitation().await {
            // Retry autojoin due to synapse sending invites, before the invited user can join. For more information, see https://github.com/matrix-org/synapse/issues/4345
            e_red_ln!("Failed to join room {err} ({:?}), retrying in {delay}s", room.room_id());

            sleep(Duration::from_secs(delay)).await;
            delay *= 2;

            if delay > 3600 {
                e_red_ln!("Can't join room {} ({err:?})", room.room_id());
                break;
            }
        }
        green_ln!("Successfully joined room {}", room.room_id());
    }
}
