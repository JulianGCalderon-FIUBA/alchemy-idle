use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::store::Store;

use super::Entity;

pub struct Gatherer {
    resources: Arc<Store>,
}

impl Gatherer {
    pub fn new(resources: &Arc<Store>) -> Self {
        Self {
            resources: resources.clone(),
        }
    }
}

impl Entity for Gatherer {
    fn rest(&mut self) {
        thread::sleep(Duration::from_secs(1));
    }

    fn work(&mut self) {
        if self.resources.extract_gold(10) {
            self.resources.add_ingredients(3);
        }
    }

    fn alive(&self) -> bool {
        true
    }
}
