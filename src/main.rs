use std::sync::Arc;

use entity::alchemist::Alchemist;
use entity::gatherer::Gatherer;
use entity::logger::Logger;
use entity::recruiter::Recruiter;
use entity::thief::Thief;
use entity::trader::Trader;
use entity::Entity;
use store::Store;
pub mod entity;
pub mod store;
pub mod utils;

const INITIAL_GOLD: usize = 100;

fn main() {
    let store = Arc::new(Store::new(INITIAL_GOLD));

    let initial_handles = vec![
        Gatherer::new(&store).start(),
        Logger::new(&store).start(),
        Alchemist::new(&store).start(),
        Trader::new(&store).start(),
        Thief::new(&store).start(),
    ];

    let recruiter_handle = Recruiter::new(&store, initial_handles).start();

    ctrlc::set_handler(move || {
        store.close();
    })
    .expect("Error setting Ctrl-C handler");

    recruiter_handle.join().expect("Should never panic")
}
