use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::store::Store;

use super::Entity;

pub struct Gatherer {
    store: Arc<Store>,
}

impl Gatherer {
    pub fn new(resources: &Arc<Store>) -> Self {
        Self {
            store: resources.clone(),
        }
    }
}

impl Entity for Gatherer {
    fn work(&mut self) {
        if self.store.extract_gold(10) {
            self.store.add_ingredients(3);
        }
    }

    fn awake(&self) -> bool {
        self.store.is_open()
    }
}
