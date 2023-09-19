use std::sync::Arc;

use crate::store::Store;
use crate::utils::random;

use super::Entity;

const MIN_GOLD_TO_STEAL: usize = 2;
const MAX_GOLD_TO_STEAL: usize = 6;

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
    fn work(&mut self) {
        let gold_to_steal = random(MIN_GOLD_TO_STEAL, MAX_GOLD_TO_STEAL);

        self.store.drain_gold(gold_to_steal);
    }

    fn awake(&self) -> bool {
        self.store.is_open()
    }
}
