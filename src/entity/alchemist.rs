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
    fn work(&mut self) {
        if self.store.extract_ingredients(3) {
            self.store.add_potions(1);
        }
    }

    fn awake(&self) -> bool {
        self.store.is_open()
    }
}
