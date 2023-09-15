use rand::random;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::store::Store;

use super::Entity;

const MAX_GOLD_TO_STEAL: usize = 25;

pub struct Thief {
    store: Arc<Store>,
}

impl Thief {
    pub fn new(resources: &Arc<Store>) -> Self {
        Self {
            store: resources.clone(),
        }
    }
}

impl Entity for Thief {
    fn rest(&mut self) {
        thread::sleep(Duration::from_secs(1));
    }

    fn work(&mut self) {
        let gold_to_steal = random::<Option<usize>>().map(|gold| gold % MAX_GOLD_TO_STEAL);

        if let Some(gold) = gold_to_steal {
            self.store.drain_gold(gold);
        }
    }

    fn alive(&self) -> bool {
        self.store.is_open()
    }
}
