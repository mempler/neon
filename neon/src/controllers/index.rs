use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

use neon_io::writer::Writer;

use crate::{enums::PacketId, events::handle_packet_stream, packets};

#[post("/")]
pub async fn index(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let writer = &mut Writer::new();

    let token_header = req.headers().get("osu-token");
    match token_header {
        Some(_) => {
            handle_packet_stream(body.to_vec().as_mut());
        },
        None => {
            // TODO: Handle login
        },
    }

    // just a test, TODO: replace with a proper packet structure.
    let announcement = packets::Announce {
        message: "Hello World".to_string(),
    };

    writer.write(&packets::Packet::new(PacketId::ServerAnnounce, announcement));

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .header("cho-token", "sample")
        .body(writer.get_data().to_owned())
}
