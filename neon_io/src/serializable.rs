use crate::{reader::Reader, writer::Writer};

pub trait Serializable {
    fn serialize(&self, writer: &mut Writer);
}

pub trait Deserializable {
    type Output<'a>: Sized;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>>;
}
