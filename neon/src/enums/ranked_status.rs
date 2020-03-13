use neon_io::{
    reader::Reader,
    serializable::{Deserializable, Serializable},
    writer::Writer,
};

#[repr(i8)]
#[derive(Copy, Clone, Debug)]
pub enum RankedStatus {
    Unknown       = -2,
    NotSubmitted  = -1,
    LatestPending = 0,
    NeedUpdate    = 1,
    Ranked        = 2,
    Approved      = 3,
    Qualified     = 4,
    Loved         = 5,
}

impl Deserializable for RankedStatus {
    type Output<'a> = RankedStatus;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        match reader.read_i8()? {
            -2 => Some(RankedStatus::Unknown),
            -1 => Some(RankedStatus::NotSubmitted),
            0 => Some(RankedStatus::LatestPending),
            1 => Some(RankedStatus::NeedUpdate),
            2 => Some(RankedStatus::Ranked),
            3 => Some(RankedStatus::Approved),
            4 => Some(RankedStatus::Qualified),
            5 => Some(RankedStatus::Loved),

            _ => None,
        }
    }
}

impl Serializable for RankedStatus {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_i8(*self as _)
    }
}
