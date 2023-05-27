use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

pub fn rand<T: Into<f64>>(rng: &mut Pcg64, lower: T, upper: T) -> f64 {
    let lower = lower.into();
    let upper = upper.into();

    rng.gen::<f64>() * (upper - lower) + lower
}

pub fn get_rng(seed: &str) -> Pcg64 {
    Seeder::from(seed).make_rng()
}
