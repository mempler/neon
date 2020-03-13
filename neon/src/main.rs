#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use neon_derive::{Deserialize, Serialize};
use neon_io::{reader::Reader, writer::Writer};

#[derive(Debug, Deserialize, Serialize)]
struct Point {
    pub x: f32,
    pub y: f32,
}

fn main() {
    let test = Point {
        x: 13.37_f32,
        y: 42.69_f32,
    };

    let mut writer = Writer::new();

    writer.write(&test);

    let mut reader = Reader::new(writer.get_data());

    println!("{:?}", reader.read::<Point>());
}
