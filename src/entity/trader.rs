use std::sync::Arc;

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
    fn work(&mut self) {
        if self.store.extract_potions(1) {
            self.store.add_gold(20);
        }
    }

    fn awake(&self) -> bool {
        self.store.is_open()
    }
}
