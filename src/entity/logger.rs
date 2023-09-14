use std::thread;
use std::time::Duration;

use crate::storage::Storage;

use super::Entity;

pub struct Logger {
    resources: Storage,
    day: usize,
}

impl Logger {
    pub fn new(resources: &Storage) -> Self {
        let resources = resources.clone();
        Self {
            resources: resources.clone(),
            day: 1,
        }
    }
}

impl Entity for Logger {
    fn rest(&mut self) {
        thread::sleep(Duration::from_secs(1));
        self.day += 1;
    }

    fn work(&mut self) {
        let gold = self.resources.gold();
        let ingredients = self.resources.ingredients();
        let potions = self.resources.potions();

        println!("Day {}:", self.day);
        println!("\tGold: {}", gold);
        println!("\tIngredients: {}", ingredients);
        println!("\tPotions: {}", potions);
    }
}
