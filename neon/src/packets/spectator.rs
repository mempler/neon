use neon_derive::{Deserialize, Serialize};
use neon_io::{reader::Reader, writer::Writer};

#[derive(Deserialize, Serialize)]
pub struct FellowSpectatorJoined {
    pub user_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct FellowSpectatorLeft {
    pub user_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct SpectatorCantSpectate {
    pub user_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct SpectatorJoined {
    pub user_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct SpectatorLeft {
    pub user_id: i32,
}
