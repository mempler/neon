#![allow(incomplete_features)]
#![feature(generic_associated_types)]

mod database;
mod enums;
mod packets;
mod settings;

use neon_derive::{Deserialize, Serialize};
use neon_io::{reader::Reader, writer::Writer};

#[derive(Debug, Deserialize, Serialize)]
struct Point {
    pub x: f32,
    pub y: f32,
}

fn main() {
    flexi_logger::Logger::with_str("off,neon=trace")
        .format(|writer, now, message| {
            write!(
                writer,
                "{} [{:>5}] {}",
                now.now().format("%Y-%m-%d %H:%M:%S"),
                message.level(),
                &message.args()
            )
        })
        .start()
        .unwrap();

    std::mem::drop(database::get_connection());
}
