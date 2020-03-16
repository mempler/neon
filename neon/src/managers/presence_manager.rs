use crate::objects::Presence;
use dashmap::{
    iter::{Iter, IterMut},
    mapref::{multiple::RefMutMulti, one::RefMut},
    DashMap,
};
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref PRESENCE_MANAGER: PresenceManager = PresenceManager::new();
}

pub struct PresenceManager {
    presences: DashMap<String, Presence>,
}

impl PresenceManager {
    pub fn new() -> Self {
        PresenceManager {
            presences: DashMap::new(),
        }
    }

    pub fn add<'a>(&'a self, presence: Presence) -> Option<RefMut<'a, String, Presence>> {
        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        self.presences.insert(uuid.clone(), presence);

        self.presences.get_mut(&uuid)
    }

    pub fn get<'a>() -> &'a Self {
        &*PRESENCE_MANAGER
    }

    pub fn get_by_token<'a>(&'a self, token: String) -> Option<RefMut<'a, String, Presence>> {
        self.presences.get_mut(&token)
    }

    pub fn get_by_username<'a>(&'a self, username: String) -> Option<RefMutMulti<'a, String, Presence>> {
        for presence in self.presences.iter_mut() {
            let presence_value = presence.value();

            if presence_value.user_info.username == username {
                return Some(presence)
            }
        }

        None
    }

    pub fn get_by_id<'a>(&'a self, id: i32) -> Option<RefMutMulti<'a, String, Presence>> {
        for presence in self.presences.iter_mut() {
            let presence_value = presence.value();

            if presence_value.user_info.id == id {
                return Some(presence)
            }
        }

        None
    }

    pub fn exists_by_token<'a>(&'a self, token: String) -> bool {
        self.presences.get_mut(&token).is_some()
    }

    pub fn exists_by_username<'a>(&'a self, username: String) -> bool {
        for presence in self.presences.iter() {
            let presence_value = presence.value();

            if presence_value.user_info.username == username {
                return true
            }
        }

        false
    }

    pub fn exists_by_id<'a>(&'a self, id: i32) -> bool {
        for presence in self.presences.iter() {
            let presence_value = presence.value();

            if presence_value.user_info.id == id {
                return true
            }
        }

        false
    }

    pub fn iter<'a, F: Fn(&'a Presence) -> bool>(&'a self, pred: F) {
        for presence in self.presences.iter() {
            if !pred(presence.value()) {
                break
            }
        }
    }

    pub fn iter_mut<'a, F: FnMut(&'a mut Presence) -> bool>(&'a self, pred: F) {
        for presence in self.presences.iter_mut() {
            if !pred(presence.value_mut()) {
                break
            }
        }
    }
}
