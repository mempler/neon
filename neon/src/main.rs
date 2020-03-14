#![allow(incomplete_features)]
#![feature(generic_associated_types)]

mod controllers;
mod database;
mod enums;
mod events;
mod packets;
mod settings;

use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    flexi_logger::Logger::with_str("off,neon=trace")
        .format(|writer, now, message| {
            write!(
                writer,
                "{} [{:>5}] {}",
                now.now().format("%Y-%m-%d %H:%M:%S"),
                message.level(),
                &message.args()
            )
        })
        .start()
        .unwrap();
    let settings = settings::get_settings();

    std::mem::drop(database::get_connection());

    let addr = settings
        .http
        .address
        .clone()
        .unwrap_or("127.0.0.1".to_string());
    let port = settings.http.port.clone().unwrap_or(5501);

    HttpServer::new(|| App::new().service(controllers::index))
        .bind(format!("{}:{}", addr, port))?
        .run()
        .await
}
