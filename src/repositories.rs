use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

pub trait EventRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: Event) -> Event;
    fn all(&self) -> Vec<Event>;
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Event {
    uid: String,
    event_name: String,
    event_detail: String,
}

impl Event {
    pub fn new(uid: String, event_name: String, event_detail: String) -> Self {
        Self {
            uid,
            event_name,
            event_detail,
        }
    }
}

type EventDatas = HashMap<i32, Event>;

#[derive(Debug, Clone)]
pub struct EventRepositoryForMemory {
    store: Arc<RwLock<EventDatas>>,
}

impl EventRepositoryForMemory {
    pub fn new() -> Self {
        EventRepositoryForMemory {
            store: Arc::default(),
        }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<EventDatas> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<EventDatas> {
        self.store.read().unwrap()
    }
}

impl EventRepository for EventRepositoryForMemory {
    fn create(&self, payload: Event) -> Event {
        let mut store = self.write_store_ref();
        let event_id = (store.len() + 1) as i32;
        let event = Event::new(
            payload.uid.clone(),
            payload.event_name.clone(),
            payload.event_detail.clone(),
        );
        store.insert(event_id, event.clone());
        event
    }

    fn all(&self) -> Vec<Event> {
        let store = self.read_store_ref();
        Vec::from_iter(store.values().map(|event| event.clone()))
    }
}
