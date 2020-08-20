

pub trait Updatable {
    fn update(&mut self, dt: f32); // dt is time in milliseconds
}