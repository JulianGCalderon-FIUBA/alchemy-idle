use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::store::Store;

use super::Entity;

pub struct Logger {
    store: Arc<Store>,
    day: usize,
}

impl Logger {
    pub fn new(resources: &Arc<Store>) -> Self {
        Self {
            store: resources.clone(),
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
        let gold = self.store.gold();
        let ingredients = self.store.ingredients();
        let potions = self.store.potions();

        println!("Day {}:", self.day);
        println!("\tGold: {}", gold);
        println!("\tIngredients: {}", ingredients);
        println!("\tPotions: {}", potions);
    }

    fn alive(&self) -> bool {
        self.store.is_open()
    }
}
