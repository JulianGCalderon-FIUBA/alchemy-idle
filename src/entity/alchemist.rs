use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::store::Store;

use super::Entity;

pub struct Alchemist {
    store: Arc<Store>,
}

impl Alchemist {
    pub fn new(resources: &Arc<Store>) -> Self {
        Self {
            store: resources.clone(),
        }
    }
}

impl Entity for Alchemist {
    fn rest(&mut self) {
        thread::sleep(Duration::from_secs(1));
    }

    fn work(&mut self) {
        if self.store.extract_ingredients(6) {
            self.store.add_potions(2);
        }
    }

    fn alive(&self) -> bool {
        self.store.is_open()
    }
}
