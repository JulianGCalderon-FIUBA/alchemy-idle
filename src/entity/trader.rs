use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::store::Store;

use super::Entity;

pub struct Trader {
    store: Arc<Store>,
}

impl Trader {
    pub fn new(resources: &Arc<Store>) -> Self {
        Self {
            store: resources.clone(),
        }
    }
}

impl Entity for Trader {
    fn rest(&mut self) {
        thread::sleep(Duration::from_secs(1));
    }

    fn work(&mut self) {
        if self.store.extract_potions(1) {
            self.store.add_gold(15);
        }
    }

    fn alive(&self) -> bool {
        self.store.is_open()
    }
}
