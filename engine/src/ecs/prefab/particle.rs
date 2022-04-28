use cgmath::prelude::*;
use specs::prelude::*;
use specs::world::LazyBuilder;
use std::time::Duration;

use crate::ecs::{ComponentCache, Particle, PrefabBuilder, SystemUtilities};
use crate::physics::{Gravity, RigidBody, TransformComponent};
use crate::utils::{QuatF, Vec3F};

pub struct ParticlePrefab {
    lifetime: Option<Duration>,
    position: Option<Vec3F>,
    velocity: Option<Vec3F>,
    gravity: Option<Gravity>,
}

impl Default for ParticlePrefab {
    fn default() -> Self {
        Self {
            lifetime: None,
            position: None,
            velocity: None,
            gravity: Some(Gravity),
        }
    }
}

impl ParticlePrefab {
    pub fn with_lifetime(mut self, lifetime: Duration) -> Self {
        self.lifetime = Some(lifetime);
        self
    }

    pub fn with_position(mut self, position: Vec3F) -> Self {
        self.position = Some(position);
        self
    }

    pub fn with_velocity(mut self, velocity: Vec3F) -> Self {
        self.velocity = Some(velocity);
        self
    }

    pub fn no_gravity(mut self) -> Self {
        self.gravity = None;
        self
    }

    fn assert_complete(&self) -> bool {
        if self.lifetime.is_none() {
            println!("WARN: Tried to build a particle without a lifetime!");
            return false;
        }
        if self.position.is_none() {
            println!("WARN: Tried to build a particle without a position!");
            return false;
        }
        if self.velocity.is_none() {
            println!("WARN: Tried to build a particle without a velocity!");
            return false;
        }
        true
    }
}

pub struct ParticleBuilder;

impl PrefabBuilder for ParticleBuilder {
    type PrefabState = ParticlePrefab;

    fn build<'a>(&mut self, api: &SystemUtilities<'a>, state: Self::PrefabState) {
        if !state.assert_complete() {
            panic!("Failed to build particle!");
        } else {
            let mut builder = api
                .entity_builder()
                .with(Particle::new(state.lifetime.unwrap()))
                .with(TransformComponent::new(
                    state.position.unwrap(),
                    Vec3F::zero(),
                    QuatF::zero(),
                ))
                .with(RigidBody::new(state.velocity.unwrap(), -Vec3F::unit_y()));
            if state.gravity.is_some() {
                builder = builder.with(Gravity);
            }
            builder.build();
        }
    }
}
