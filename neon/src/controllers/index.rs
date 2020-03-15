use actix_web::{web::Bytes, HttpRequest, HttpResponse, Responder};

use crate::{
    events::{handle_login_request, handle_packet_stream},
    objects::Presence,
};

#[actix_web::post("/")]
pub async fn index(req: HttpRequest, body: Bytes) -> impl Responder {
    // let mut writer = Writer::new();

    let mut presence = Presence::new();
    if let Some(_token) = req.headers().get("osu-token") {
        // TODO: replace with "getting presence from presence array otherwise create new presence"
        handle_packet_stream(&mut presence, &body);
    } else {
        presence = handle_login_request(&body);
    }

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .header("cho-protocol", "19")
        .header("cho-token", "sample")
        .body(presence.get_data().to_vec())
}
