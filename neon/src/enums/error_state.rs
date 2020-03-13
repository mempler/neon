use neon_io::{
    reader::Reader,
    serializable::{Deserializable, Serializable},
    writer::Writer,
};

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum ErrorState {
    Ok     = 0,
    Update = 1,
}

impl Deserializable for ErrorState {
    type Output<'a> = ErrorState;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        match reader.read_u8()? {
            0 => Some(ErrorState::Ok),
            1 => Some(ErrorState::Update),
            _ => None,
        }
    }
}

impl Serializable for ErrorState {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_u8(*self as _)
    }
}
