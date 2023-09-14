use std::thread;
use std::time::Duration;

use crate::storage::Storage;

pub struct Gatherer {
    _resources: Storage,
}

impl Gatherer {
    pub fn new(resources: &Storage) -> Self {
        let resources = resources.clone();
        Self {
            _resources: resources.clone(),
        }
    }

    pub fn start(self) -> thread::JoinHandle<()> {
        thread::spawn(move || self.work())
    }

    pub fn work(self) {
        loop {
            println!("Gatherer: I'm going to work!");
            thread::sleep(Duration::from_secs(1));
        }
    }
}
