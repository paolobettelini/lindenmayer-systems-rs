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

pub fn rgb(r: f64, g: f64, b: f64) -> f64 {
    let r = u64::min(255, u64::max(0, r as u64));
    let g = u64::min(255, u64::max(0, g as u64));
    let b = u64::min(255, u64::max(0, b as u64));

    let color = r | (g << 8) | (b << 16);
    color as f64
}