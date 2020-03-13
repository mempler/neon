use neon_derive::{Deserialize, Serialize};
use neon_io::{reader::Reader, writer::Writer};

#[derive(Deserialize, Serialize)]
pub struct MatchPlayerFailed {
    pub slot_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct MatchPlayerSkipped {
    pub slot_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct MatchChangeMods {
    pub mods: u32,
}

#[derive(Deserialize, Serialize)]
pub struct MatchChangeSlot {
    pub slot_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct MatchLock {
    pub slot_id: u32,
}

#[derive(Deserialize, Serialize)]
pub struct MatchTransferHost {
    pub slot_id: u32,
}

#[derive(Deserialize, Serialize)]
pub struct MatchJoin {
    pub match_id: i32,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClientInvite {
    pub user_id: i32,
}
