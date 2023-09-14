use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Storage {
    _gold: Arc<RwLock<usize>>,
    _ingredients: Arc<RwLock<usize>>,
    _potions: Arc<RwLock<usize>>,
}

impl Storage {
    pub fn new(initial_gold: usize) -> Self {
        Self {
            _gold: Arc::new(RwLock::new(initial_gold)),
            _ingredients: Arc::new(RwLock::new(0)),
            _potions: Arc::new(RwLock::new(0)),
        }
    }
}
