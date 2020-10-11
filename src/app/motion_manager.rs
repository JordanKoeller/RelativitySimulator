use utils::*;
use cgmath::prelude::*;
use renderer::LIGHT_SPEED;

const MAX_ACCELERATION: f32 = 6f32;

pub struct PlayerMotionDelegate {
    max_beta: f32,
    pub velocity: Vec3F,
    acceleration: Vec3F,
    max_acceleration: f32
}

impl Default for PlayerMotionDelegate {
    fn default() -> Self {
        PlayerMotionDelegate {
            max_beta: 0.8,
            velocity: Vec3F::new(0.0, 0.0, 0.0),
            acceleration: Vec3F::new(0.0, 0.0, 0.0),
            max_acceleration: MAX_ACCELERATION
        }
    }
}

impl PlayerMotionDelegate {
    fn compute_drag(&self) -> f32 {
        // f_m = D * MAX_SPEED * MAX_SPEED
        // D = f_m / (MAX_SPEED)^2
        let max_speed = self.max_beta * LIGHT_SPEED;
        let drag = self.max_acceleration / (max_speed * max_speed);
        drag
    }

    fn speed(&self) -> f32 {
        self.velocity.magnitude()
    }

    pub fn increment_max_speed(&mut self) {
        let diff = 1.0 - self.max_beta;
        let max_diff = diff / 2.0;
        let speed_increment: f32 = 0.1;
        self.max_beta += speed_increment.max(max_diff);
    }

    pub fn decrement_max_speed(&mut self) {
        let diff = 1.0 - self.max_beta;
        let max_diff = diff * 2.0;
        let speed_increment: f32 = 0.1;
        self.max_beta -= speed_increment.min(max_diff);
    }

    pub fn apply_user_acceleration(&mut self, acc: Vec3F) {
        self.acceleration += acc;
    }

    pub fn velocity(&self) -> &Vec3F {
        &self.velocity
    }

    pub fn apply_brakes(&mut self) {
        let speed = self.speed();
        if self.speed() < 0.1 {
            self.velocity = Vec3F::new(0.0, 0.0, 0.0);
        } else {
            let brake_direction = - self.velocity().normalize();
            self.velocity += speed * 0.05 * brake_direction;
        }
    }

    pub fn update(&mut self, dt: f32) {
        let norm_acc = if self.acceleration.magnitude() != 0.0 {self.acceleration.normalize_to(self.max_acceleration)} else {self.acceleration};
        if self.velocity.magnitude() > 0.1 {
            let drag_acc = self.compute_drag() * self.speed() * self.speed();
            let total_acc = norm_acc - self.velocity.normalize() * drag_acc;
            self.velocity += total_acc * dt;
        } else if norm_acc.magnitude() > 0.1 {
            self.velocity += norm_acc * dt;
        }
        self.acceleration = Vec3F::new(0.0, 0.0, 0.0)
    }
}