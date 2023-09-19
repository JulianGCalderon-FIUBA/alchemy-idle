use std::ops::{Add, Rem, Sub};

use rand::distributions::Standard;
use rand::prelude::Distribution;

pub fn random<T>(min: T, max: T) -> T
where
    Standard: Distribution<T>,
    T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + Copy,
{
    rand::random() % (max - min) + min
}
