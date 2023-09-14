use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Storage {
    gold: Arc<RwLock<usize>>,
    ingredients: Arc<RwLock<usize>>,
    potions: Arc<RwLock<usize>>,
}

impl Storage {
    pub fn new(initial_gold: usize) -> Self {
        Self {
            gold: Arc::new(RwLock::new(initial_gold)),
            ingredients: Arc::new(RwLock::new(0)),
            potions: Arc::new(RwLock::new(0)),
        }
    }

    pub fn gold(&self) -> usize {
        self.gold.read().unwrap().clone()
    }

    pub fn extract_gold(&self, amount: usize) -> bool {
        let mut gold = self.gold.write().unwrap();

        if *gold < amount {
            return false;
        }

        *gold -= amount;

        true
    }

    pub fn ingredients(&self) -> usize {
        self.ingredients.read().unwrap().clone()
    }

    pub fn add_ingredients(&self, amount: usize) {
        let mut ingredients = self.ingredients.write().unwrap();

        *ingredients += amount;
    }

    pub fn potions(&self) -> usize {
        self.potions.read().unwrap().clone()
    }
}
