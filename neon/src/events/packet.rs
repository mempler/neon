use neon_io::reader::Reader;

use crate::{
    enums::PacketId,
    objects::Presence,
    packets::{Message, Packet},
};

pub fn handle_packet_stream(presence: &mut Presence, input_data: &[u8]) -> Option<()> {
    let mut reader = Reader::new(input_data);

    while reader.available() >= 7 {
        let packet_id = reader.read::<PacketId>()?;

        reader.seek_backward(2);

        match packet_id {
            PacketId::ClientSendIrcMessage => {
                let message = reader.read::<Packet<Message>>()?;

                log::info!("Incoming chat message: {:?}", &message.data);
            },
            _ => {
                let packet = reader.read::<Packet<()>>()?;

                log::warn!("Unhandled packet: {:?}.", packet.id);
            },
        }
    }

    Some(())
}
