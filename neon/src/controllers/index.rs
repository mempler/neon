use actix_web::{http::ContentEncoding, post, web, HttpResponse, Responder};

use neon_io::writer::Writer;

use crate::{enums::PacketId, packets};

#[post("/")]
pub async fn index(info: web::Path<()>) -> impl Responder {
    let writer = &mut Writer::new();
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
