use neon_derive::{Deserialize, Serialize};
use neon_io::{reader::Reader, writer::Writer};

#[derive(Deserialize, Serialize)]
pub struct Channel {
    pub name:       String,
    pub topic:      String,
    pub user_count: u16,
}

#[derive(Deserialize, Serialize)]
pub struct ChannelJoinSuccess {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChannelRevoked {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChannelJoin {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChannelLeave {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
    pub username:       String,
    pub message:        String,
    pub channel_target: String,
    pub sender_id:      i32,
}
