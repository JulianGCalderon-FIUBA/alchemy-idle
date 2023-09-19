use std::thread;
use std::time::Duration;

use rand::distributions::Standard;
use rand::prelude::Distribution;

use crate::utils::random;

pub mod alchemist;
pub mod gatherer;
pub mod logger;
pub mod recruiter;
pub mod thief;
pub mod trader;

#[derive(Debug)]
enum RecruitableEntity {
    Gatherer,
    Alchemist,
    Trader,
    Thief,
}

impl Distribution<RecruitableEntity> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> RecruitableEntity {
        match rng.gen_range(0..=3) {
            0 => RecruitableEntity::Gatherer,
            1 => RecruitableEntity::Alchemist,
            2 => RecruitableEntity::Trader,
            3 => RecruitableEntity::Thief,
            _ => unreachable!(),
        }
    }
}

pub trait Entity: Sized + Send + 'static {
    fn work(&mut self);
    fn awake(&self) -> bool;
    fn finally(self) {}

    fn rest(&mut self) {
        thread::sleep(Duration::from_millis(random(500, 1500)));
    }

    fn start(mut self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            while self.awake() {
                self.work();
                self.rest();
            }
            self.finally();
        })
    }
}
