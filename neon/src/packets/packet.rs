use neon_io::{
    reader::Reader,
    serializable::{Deserializable, Serializable},
    writer::Writer,
};

use crate::enums::PacketId;

pub struct Packet<T: Sized> {
    pub id:   PacketId,
    pub data: T,
}

impl<T: Sized + Serializable + Deserializable> Deserializable for Packet<T> {
    type Output<'a> = Packet<T::Output<'a>>;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        let id = reader.read::<PacketId>()?;
        let _ = reader.read_u8()?;
        let length = reader.read_u32()?;
        let data = reader.read_bytes(length as _)?;

        let mut data_reader = Reader::new(data);

        Some(Packet {
            id,
            data: data_reader.read::<T>()?,
        })
    }
}

impl<T: Sized + Serializable + Deserializable> Serializable for Packet<T> {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_u16(*&self.id as _);
        writer.write_u8(0);

        let t_writer = &mut Writer::new();
        t_writer.write(&self.data);

        writer.write_i32(t_writer.get_data().len() as _);
        writer.write_bytes(t_writer.get_data());
    }
}

impl<T: Sized> Packet<T> {
    pub fn new(id: PacketId, data: T) -> Self {
        Packet {
            id,
            data,
        }
    }
}
