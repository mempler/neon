use crate::database::models::UserModel;
use neon_io::{serializable::Serializable, writer::Writer};

pub struct Presence {
    pub user_info: UserModel,

    // This is used so we can know if a presense is logged in otherwise this presence will be used as a "slot" to increase
    // the response time at login.
    // this needs to be True if given user is connected to Bancho
    pub is_logged_in: bool,
    writer:           Writer,
}

impl Presence {
    pub fn new() -> Self {
        Presence {
            user_info: UserModel {
                id:                0,
                username:          "Not Loggedin".to_string(),
                password_md5:      "".to_string(),
                email:             "".to_string(),
                register_datetime: 0,
                latest_activity:   0,
                privileges:        0,
                donor_expire:      0,
            },

            is_logged_in: false,
            writer:       Writer::new(),
        }
    }

    pub fn clear(&mut self) {
        self.writer.clear();
    }

    pub fn get_data(&self) -> &[u8] {
        self.writer.get_data()
    }

    pub fn write<T: ?Sized>(&mut self, v: &T)
    where T: Serializable {
        self.writer.write(v);
    }
}
