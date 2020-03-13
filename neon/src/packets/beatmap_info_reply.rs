use neon_derive::{Deserialize, Serialize};
use neon_io::{reader::Reader, writer::Writer};

use crate::enums::{Rank, RankedStatus};

#[derive(Deserialize, Serialize)]
pub struct BeatmapInfoReply {
    pub beatmap_id:    u32,
    pub set_id:        u32,
    pub ranked_status: RankedStatus,
    pub osu_rank:      Rank,
    pub taiko_rank:    Rank,
    pub catch_rank:    Rank,
    pub mania_rank:    Rank,
    pub file_md5:      String,
}
