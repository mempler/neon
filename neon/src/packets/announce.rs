use neon_derive::{Deserialize, Serialize};
use neon_io::{reader::Reader, writer::Writer};

#[derive(Deserialize, Serialize)]
pub struct Announce {
    pub message: String,
}
