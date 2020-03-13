use neon_io::{
    reader::Reader,
    serializable::{Deserializable, Serializable},
    writer::Writer,
};

bitflags::bitflags! {
    pub struct Permissions: i32 {
        const USER          = 1 << 0;
        const BAT           = 1 << 1;
        const SUPPORTER     = 1 << 2;
        const MODERATOR     = 1 << 3;
        const DEVELOPER     = 1 << 4;
        const ADMINISTRATOR = 1 << 5;
        const TOURNEY_STAFF = 1 << 6;
    }
}

impl Deserializable for Permissions {
    type Output<'a> = Permissions;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        Some(Permissions {
            bits: reader.read_i32()?,
        })
    }
}

impl Serializable for Permissions {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_i32(self.bits as _)
    }
}
