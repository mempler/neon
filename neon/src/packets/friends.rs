use neon_derive::{Deserialize, Serialize};
use neon_io::{reader::Reader, writer::Writer};

#[derive(Deserialize, Serialize)]
pub struct FriendAdd {
    pub user_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct FriendRemove {
    pub user_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct FriendsList {
    pub user_ids: Vec<i32>,
}
