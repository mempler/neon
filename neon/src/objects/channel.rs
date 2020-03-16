use crate::{
    enums::PacketId,
    managers::PresenceManager,
    objects::Presence,
    packets::{self, Message},
};

#[derive(Clone, Debug)]
pub struct Channel {
    pub name:  String,
    pub topic: String,

    users: Vec<i32>,
}

impl Channel {
    pub fn new(name: String, topic: String) -> Self {
        Channel {
            name,
            topic,
            users: Vec::new(),
        }
    }

    pub fn send_message(&mut self, message: Message) -> bool {
        if !self.user_joined(message.sender_id.clone()) {
            return false
        }

        PresenceManager::get().iter_mut(|presence| -> bool {
            if presence.user_info.id == message.sender_id {
                return true
            }

            presence.write(&packets::Packet::new(PacketId::ServerSendMessage, message));

            true
        });

        true
    }

    pub fn get_user_count(&self) -> usize {
        self.users.len()
    }

    pub fn user_joined(&self, user: i32) -> bool {
        self.users.iter().any(|u| *u == user)
    }

    pub fn join(&mut self, user: i32) {
        self.users.push(user);

        match PresenceManager::get().get_by_id(user) {
            Some(kv) => kv.value_mut().write(&packets::Packet::new(
                PacketId::ServerChannelJoinSuccess,
                packets::ChannelJoinSuccess {
                    name: self.name
                },
            )),
            None => {},
        }
    }

    pub fn leave(&mut self, user: i32) {
        self.users.remove_item(&user);

        match PresenceManager::get().get_by_id(user) {
            Some(kv) => kv.value_mut().write(&packets::Packet::new(
                PacketId::ServerChannelRevoked,
                packets::ChannelRevoked {
                    name: self.name
                },
            )),
            None => {},
        }
    }
}
