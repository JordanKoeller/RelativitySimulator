use cgmath::prelude::*;
use specs::prelude::*;
use specs::world::LazyBuilder;
use std::time::Duration;

use crate::ecs::{Particle, PrefabBuilder,};
use crate::physics::{Gravity, TransformComponent, RigidBody};
use crate::utils::{Vec3F, QuatF};

pub struct ParticleBuilder<B: PrefabBuilder> {
    builder: Option<B>,
    lifetime: Option<Duration>,
    position: Option<Vec3F>,
    velocity: Option<Vec3F>,
    gravity: Option<Gravity>,
    state: Option<B::PrefabState>,


}

impl<B: PrefabBuilder> Default for ParticleBuilder<B> {
    fn default() -> Self {
        Self {
            builder: None,
            lifetime: None,
            position: None,
            velocity: None,
            gravity: Some(Gravity),
            state: None,
        }
    }
}

impl<B: PrefabBuilder> ParticleBuilder<B> {

    pub fn with_builder(mut self, builder: B) -> Self {
        self.builder = Some(builder);
        self
    }

    pub fn with_prefab_state(mut self, state: B::PrefabState) -> Self {
        self.state = Some(state);
        self
    }

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

    pub fn build(self, mut builder: LazyBuilder<'_>) -> LazyBuilder<'_> {
        if !self.assert_complete() {
            panic!("Failed to build particle!");
        } else {
            builder = builder
                .with(Particle::new(self.lifetime.unwrap()))
                .with(TransformComponent::new(
                    self.position.unwrap(),
                    Vec3F::zero(),
                    QuatF::zero(),
                ))
                .with(RigidBody::new(self.velocity.unwrap(), -Vec3F::unit_y()));
            if self.builder.is_some() && self.state.is_some() {
                builder = self.builder.unwrap().build(builder, self.state.unwrap());
            }
            if self.gravity.is_some() {
                builder = builder.with(Gravity);
            }
            builder

        }
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
        if self.builder.is_some() != self.state.is_some() {
            println!("WARN: Tried to build a particle with only a builder and no state or vice versa");
            return false;
        }
        true
    }
}