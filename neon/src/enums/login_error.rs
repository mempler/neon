use neon_io::{
    reader::Reader,
    serializable::{Deserializable, Serializable},
    writer::Writer,
};

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum LoginError {
    Failed        = -1,
    Outdated      = -2,
    Banned        = -3,
    MultiAcc      = -4,
    Exception     = -5,
    SupporterOnly = -6,
    PasswordReset = -7,
    TwoFactorAuth = -8,
}

impl Deserializable for LoginError {
    type Output<'a> = LoginError;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        match reader.read_i32()? {
            -1 => Some(LoginError::Failed),
            -2 => Some(LoginError::Outdated),
            -3 => Some(LoginError::Banned),
            -4 => Some(LoginError::MultiAcc),
            -5 => Some(LoginError::Exception),
            -6 => Some(LoginError::SupporterOnly),
            -7 => Some(LoginError::PasswordReset),
            -8 => Some(LoginError::TwoFactorAuth),
            _ => None,
        }
    }
}

impl Serializable for LoginError {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_i32(*self as _)
    }
}
