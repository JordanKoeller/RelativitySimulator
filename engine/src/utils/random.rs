use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

// Return a random float between start and end
pub fn rand_float(start: f64, end: f64) -> f64 {
    thread_rng().gen_range(start, end)
}

// Return a boolean with an asymmetric likelihood
pub fn rand_choice(probability: f64) -> bool {
    thread_rng().gen_range(0f64, 1f64) < probability
}

pub fn rand_ind(start: usize, end: usize) -> usize {
    thread_rng().gen_range(start, end)
}
