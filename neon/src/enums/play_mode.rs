use neon_io::{
    reader::Reader,
    serializable::{Deserializable, Serializable},
    writer::Writer,
};

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum PlayMode {
    Std          = 0,
    Taiko        = 1,
    CatchTheBeat = 2,
    Mania        = 3,
}

impl Deserializable for PlayMode {
    type Output<'a> = PlayMode;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        match reader.read_u8()? {
            0 => Some(PlayMode::Std),
            1 => Some(PlayMode::Taiko),
            2 => Some(PlayMode::CatchTheBeat),
            3 => Some(PlayMode::Mania),
            _ => None,
        }
    }
}

impl Serializable for PlayMode {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_u8(*self as _)
    }
}
