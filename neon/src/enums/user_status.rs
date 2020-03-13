use neon_io::{
    reader::Reader,
    serializable::{Deserializable, Serializable},
    writer::Writer,
};

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum UserAction {
    Idle         = 0,
    Afk          = 1,
    Playing      = 2,
    Editing      = 3,
    Modding      = 4,
    Multiplayer  = 5,
    Watching     = 6,
    Unknown      = 7,
    Testing      = 8,
    Submitting   = 9,
    Paused       = 10,
    Lobby        = 11,
    Multiplaying = 12,
    OsuDirect    = 13,
}

impl Deserializable for UserAction {
    type Output<'a> = UserAction;

    fn deserialize<'a>(reader: &mut Reader<'a>) -> Option<Self::Output<'a>> {
        match reader.read_i8()? {
            0 => Some(UserAction::Idle),
            1 => Some(UserAction::Afk),
            2 => Some(UserAction::Playing),
            3 => Some(UserAction::Editing),
            4 => Some(UserAction::Modding),
            5 => Some(UserAction::Multiplayer),
            6 => Some(UserAction::Watching),
            7 => Some(UserAction::Unknown),
            8 => Some(UserAction::Testing),
            9 => Some(UserAction::Submitting),
            10 => Some(UserAction::Paused),
            11 => Some(UserAction::Lobby),
            12 => Some(UserAction::Multiplaying),
            13 => Some(UserAction::OsuDirect),
            _ => None,
        }
    }
}

impl Serializable for UserAction {
    fn serialize(&self, writer: &mut Writer) {
        writer.write_i8(*self as _)
    }
}
