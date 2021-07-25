use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};


// Return a random float between start and end
pub fn rand_float(start: f32, end: f32) -> f32 {
  thread_rng().gen_range(start, end)
}

// Return a boolean with an asymmetric likelihood
pub fn rand_choice(probability: f32) -> bool {
  thread_rng().gen_range(0f32, 1f32) < probability
} 

pub fn rand_ind(start: usize, end: usize) -> usize {
  thread_rng().gen_range(start, end)
}