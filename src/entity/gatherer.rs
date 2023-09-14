use std::thread;
use std::time::Duration;

use crate::storage::Storage;

use super::Entity;

pub struct Gatherer {
    resources: Storage,
}

impl Gatherer {
    pub fn new(resources: &Storage) -> Self {
        let resources = resources.clone();
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
}
