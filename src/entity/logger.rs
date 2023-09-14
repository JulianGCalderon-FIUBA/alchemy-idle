use std::thread;
use std::time::Duration;

use crate::storage::Storage;

use super::Entity;

pub struct Logger {
    _resources: Storage,
}

impl Logger {
    pub fn new(resources: &Storage) -> Self {
        let resources = resources.clone();
        Self {
            _resources: resources.clone(),
        }
    }
}

impl Entity for Logger {
    fn rest(&mut self) {
        thread::sleep(Duration::from_secs(1));
    }

    fn work(&mut self) {
        println!("Logger: I'm going to work!");
    }
}
