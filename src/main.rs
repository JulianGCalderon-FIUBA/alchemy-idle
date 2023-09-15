use std::sync::Arc;

use entity::gatherer::Gatherer;
use entity::logger::Logger;
use entity::Entity;
use store::Store;

pub mod entity;
pub mod store;

const INITIAL_GOLD: usize = 100;

fn main() {
    let store = Arc::new(Store::new(INITIAL_GOLD));

    let handles = vec![Gatherer::new(&store).start(), Logger::new(&store).start()];

    ctrlc::set_handler(move || {
        store.close();
    })
    .expect("Error setting Ctrl-C handler");

    for handle in handles {
        handle.join().unwrap();
    }
}
