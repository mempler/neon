use neon_io::reader::Reader;

use crate::enums::PacketId;

pub fn handle_packet_stream<'a>(input_data: &'a [u8]) {
    let reader = &mut Reader::new(input_data);

    while reader.available() > 0 {
        let packet_id = reader.read::<PacketId>().unwrap();

        reader.seek_backward(2);
        match packet_id {
            _ => println!("Unhandled Packet {:?}", packet_id),
        }
    }
}
