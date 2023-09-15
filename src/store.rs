use std::sync::atomic::AtomicBool;
use std::sync::RwLock;

pub struct Store {
    gold: RwLock<usize>,
    ingredients: RwLock<usize>,
    potions: RwLock<usize>,
    open: AtomicBool,
}

impl Store {
    pub fn new(initial_gold: usize) -> Self {
        Self {
            gold: RwLock::new(initial_gold),
            ingredients: RwLock::new(0),
            potions: RwLock::new(0),
            open: AtomicBool::new(true),
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

    pub fn close(&self) {
        self.open.store(false, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn is_open(&self) -> bool {
        self.open.load(std::sync::atomic::Ordering::Relaxed)
    }
}
