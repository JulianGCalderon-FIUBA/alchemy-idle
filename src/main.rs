use entity::gatherer::Gatherer;
use storage::Storage;

pub mod entity;
pub mod storage;

const INITIAL_GOLD: usize = 100;

fn main() {
    let storage = Storage::new(INITIAL_GOLD);

    let gatherer = Gatherer::new(&storage);

    let handle = gatherer.start();

    handle.join().unwrap();
}
