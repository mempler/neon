use crate::objects::Channel;
use dashmap::{mapref::one::RefMut, DashMap};

lazy_static::lazy_static! {
    static ref CHANNEL_MANAGER: ChannelManager = ChannelManager::new();
}

pub struct ChannelManager {
    channels: DashMap<String, Channel>,
}

impl ChannelManager {
    pub fn new() -> Self {
        ChannelManager {
            channels: DashMap::new(),
        }
    }

    pub fn add<'a>(&'a self, channel: Channel) -> Option<RefMut<'a, String, Channel>> {
        self.channels.insert(channel.name.clone(), channel.clone());

        self.channels.get_mut(&channel.name)
    }

    pub fn get<'a>() -> &'a Self {
        &*CHANNEL_MANAGER
    }

    pub fn get_by_name<'a>(&'a self, name: String) -> Option<RefMut<'a, String, Channel>> {
        self.channels.get_mut(&name)
    }

    pub fn exists_by_name<'a>(&'a self, name: String) -> bool {
        for channel in self.channels.iter() {
            let channel_value = channel.value();

            if channel_value.name == name {
                return true
            }
        }

        false
    }

    pub fn iter<'a, F: Fn(&'a Channel) -> bool>(&'a self, pred: F) {
        for channel in self.channels.iter() {
            if !pred(channel.value()) {
                break
            }
        }
    }

    pub fn iter_mut<'a, F: FnMut(&'a mut Channel) -> bool>(&'a self, pred: F) {
        for channel in self.channels.iter_mut() {
            if !pred(channel.value_mut()) {
                break
            }
        }
    }
}
