use crate::{
    database::{get_connection, models::UserModel, schema::users},
    enums::{LoginError, PacketId},
    objects::Presence,
    packets,
    packets::Packet,
};
use diesel::prelude::*;

pub fn handle_login_request(input_data: &[u8]) -> Presence {
    let mut presence = Presence::new();

    let data_string = String::from_utf8(input_data.to_vec()).unwrap();
    let login_information: Vec<&str> = data_string.splitn(3, "\n").collect();

    let user_name = login_information[0];
    let _password = login_information[1];

    let connection = get_connection();
    let results = users::table
        .filter(users::username.eq(user_name))
        .limit(1)
        .load::<UserModel>(&connection);

    log::info!("{:?}", results);

    // let user_info: UserModel;

    // if results

    // let mut presence = Presence::new(results);

    let system_information: Vec<&str> = login_information[2].split("|").collect();

    if system_information.len() < 5 {
        presence.clear(); // Clear all already written packets

        presence.write(&packets::Packet::new(PacketId::ServerLoginResponse, packets::LoginResponse {
            response: LoginError::Exception as _,
        }));

        return presence
    }

    let _osu_build = system_information[0];
    let _time_zone: i8 = system_information[1].parse().unwrap();
    let _display_location = system_information[2] == "1";
    let _security_information = system_information[3]; // TODO: use this to prevent Multi accounting
    let _block_non_friends_dm = system_information[4] == "1";

    presence
}
