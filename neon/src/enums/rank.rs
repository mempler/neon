use neon_io::{
    reader::Reader,
    serializable::{Deserializable, Serializable},
    writer::Writer,
};

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Rank {
    XH = 0,
    SH = 1,
    X  = 2,
    S  = 3,
    A  = 4,
    B  = 5,
    C  = 6,
    D  = 7,
    N  = 10,
}

impl Deserializable for Rank {
    type Output<'a> = Rank;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        match reader.read_u8()? {
            0 => Some(Rank::XH),
            1 => Some(Rank::SH),
            2 => Some(Rank::X),
            3 => Some(Rank::S),
            4 => Some(Rank::A),
            5 => Some(Rank::B),
            6 => Some(Rank::C),
            7 => Some(Rank::D),
            10 => Some(Rank::N),
            _ => None,
        }
    }
}

impl Serializable for Rank {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_u8(*self as _)
    }
}
