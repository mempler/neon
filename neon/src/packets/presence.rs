use neon_derive::{Deserialize, Serialize};
use neon_io::{reader::Reader, writer::Writer};

use crate::enums::{Country, ErrorState, LoginError, Permissions, PlayMode, UserAction};

#[derive(Deserialize, Serialize)]
pub struct LoginPermissions {
    pub permission: Permissions,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub response: LoginError,
}

#[derive(Deserialize, Serialize)]
pub struct PresenceBundle {
    pub user_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct PresenceSingle {
    pub user_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct UserStatus {
    pub action:           UserAction,
    pub action_text:      String,
    pub beatmap_checksum: String,
    pub current_mods:     u32,
    pub play_mode:        PlayMode,
    pub beatmap_id:       u32,
}

#[derive(Deserialize, Serialize)]
pub struct UserStats {
    pub ranked_score: u64,
    pub accuracy:     f32,
    pub play_count:   i32,
    pub total_score:  u64,
    pub position:     i32,
    pub performance:  i16,
}

#[derive(Deserialize, Serialize)]
pub struct HandlePresenceUpdate {
    pub user_id: i32,
    pub status:  UserStatus,
    pub stats:   UserStats,
}

#[derive(Deserialize, Serialize)]
pub struct HandleUserQuit {
    pub user_id:     i32,
    pub error_state: ErrorState,
}

#[derive(Deserialize, Serialize)]
pub struct Presence {
    pub user_id:          i32,
    pub user_name:        String,
    pub time_zone:        u8,
    pub country:          Country,
    pub permission:       Permissions,
    pub longitude:        f32,
    pub latitude:         f32,
    pub ranking_position: i32,
}
