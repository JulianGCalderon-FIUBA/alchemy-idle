use entity::gatherer::Gatherer;
use entity::logger::Logger;
use entity::Entity;
use storage::Storage;

pub mod entity;
pub mod storage;

const INITIAL_GOLD: usize = 100;

fn main() {
    let storage = Storage::new(INITIAL_GOLD);

    let handles = vec![
        Gatherer::new(&storage).start(),
        Logger::new(&storage).start(),
    ];

    for handle in handles {
        handle.join().unwrap();
    }
}
