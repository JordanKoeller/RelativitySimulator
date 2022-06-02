use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::physics::{CanCollide, Collision, CollisionQueue, CollisionSummary};
use crate::utils::*;
use cgmath::prelude::*;

use crate::physics::{AxisAlignedCubeCollision, Drag, Gravity, RigidBody, TransformComponent};

use crate::renderer::LIGHT_SPEED;
const MAX_ACCELERATION: f64 = 6f64;

const DRAG: f64 = MAX_ACCELERATION / LIGHT_SPEED / LIGHT_SPEED;
const GRAVITY: f64 = 14f64;

const ACCELERATION_MIN_MAGNITUDE: f64 = 0.00000001;

pub struct MotionSystem;

impl<'a> System<'a> for MotionSystem {
    type SystemData = (
        WriteStorage<'a, TransformComponent>,
        WriteStorage<'a, RigidBody>,
        ReadStorage<'a, CanCollide>,
        ReadStorage<'a, Gravity>,
        ReadStorage<'a, Drag>,
        ReadStorage<'a, AxisAlignedCubeCollision>,
        Read<'a, Timestep>,
    );

    fn run(
        &mut self,
        (mut transform_s, mut rigid_storage, collidable_storage, gravity_storage, drag_storage, _colliders_storage, dt): Self::SystemData,
    ) {
        for (transform, _collidable, rigid_body, gravity, drag) in (
            &mut transform_s,
            (&collidable_storage).maybe(),
            &mut rigid_storage,
            (&gravity_storage).maybe(),
            (&drag_storage).maybe(),
        )
            .join()
        {
            self.compute_kinematics(rigid_body, gravity, drag, dt.dt().as_secs_f64() as f64);
            // println!("Pushing update");
            self.push_frame_update(rigid_body, transform, dt.dt().as_secs_f64() as f64);
            rigid_body.reset_acceleration();
        }
    }
}

impl MotionSystem {
    fn compute_kinematics(&self, rigid_body: &mut RigidBody, gravity: Option<&Gravity>, drag: Option<&Drag>, dt: f64) {
        if let Some(_g) = gravity {
            rigid_body.acceleration -= Vec3F::unit_y() * GRAVITY;
        }
        if let Some(_d) = drag {
            if rigid_body.velocity.magnitude2() > 0.1 {
                rigid_body.acceleration -= DRAG * rigid_body.velocity.magnitude2() * rigid_body.velocity.normalize();
            }
        }
        self.integrate_rigid_body(rigid_body, dt);
    }

    fn integrate_rigid_body(&self, rigid_body: &mut RigidBody, dt: f64) {
        // Linear first
        if rigid_body.acceleration.magnitude2() > ACCELERATION_MIN_MAGNITUDE {
            rigid_body.velocity += rigid_body.acceleration * dt;
        }

        // Angular
        if rigid_body.angular_acceleration.magnitude2() > ACCELERATION_MIN_MAGNITUDE {
            rigid_body.angular_velocity = rigid_body.angular_velocity * rigid_body.angular_acceleration * dt;
        }
    }

    // Returns a vector and quaternion, representing how far this RigidBody
    // has translated and rotated over this frame.
    fn push_frame_update(&self, rigid_body: &RigidBody, transform: &mut TransformComponent, dt: f64) {
        transform.push_translation(rigid_body.velocity * dt);
        transform.push_rotation(&(rigid_body.angular_velocity * dt));
    }
}
