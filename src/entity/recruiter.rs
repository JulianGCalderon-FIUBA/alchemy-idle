use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::store::Store;
use crate::utils::random;

use super::alchemist::Alchemist;
use super::gatherer::Gatherer;
use super::thief::Thief;
use super::trader::Trader;
use super::{Entity, RecruitableEntity};

const RECRUIT_PRICE: usize = 100;

pub struct Recruiter {
    store: Arc<Store>,
    handles: Vec<JoinHandle<()>>,
}

impl Recruiter {
    pub fn new(store: &Arc<Store>, initial_handles: Vec<JoinHandle<()>>) -> Self {
        Self {
            store: store.clone(),
            handles: initial_handles,
        }
    }

    fn spawn_entity(&mut self, entity: impl Entity) {
        self.handles.push(entity.start())
    }
}

impl Entity for Recruiter {
    fn work(&mut self) {
        if self.store.gold() < RECRUIT_PRICE * 2 {
            return;
        }

        if self.store.extract_gold(RECRUIT_PRICE) {
            let to_recruit = rand::random::<RecruitableEntity>();
            println!("Recruiting a {:#?}", to_recruit);

            match to_recruit {
                RecruitableEntity::Gatherer => self.spawn_entity(Gatherer::new(&self.store)),
                RecruitableEntity::Alchemist => self.spawn_entity(Alchemist::new(&self.store)),
                RecruitableEntity::Trader => self.spawn_entity(Trader::new(&self.store)),
                RecruitableEntity::Thief => self.spawn_entity(Thief::new(&self.store)),
            }
        }
    }

    fn awake(&self) -> bool {
        self.store.is_open()
    }

    fn finally(self) {
        for handle in self.handles {
            handle.join().expect("Should never panic")
        }
    }
}
