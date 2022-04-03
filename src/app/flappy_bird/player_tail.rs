use cgmath::prelude::*;
use rand::{thread_rng, Rng};
use specs::prelude::*;
use std::time::Duration;

use ecs::*;
use renderer::{Drawable, Mesh, Renderer, Texture};
use shapes::Sprite;
use utils::*;

use app::{AxisAlignedCubeCollision, Cube};
use physics::{Gravity, RigidBody, TransformComponent};

#[derive(Clone, Debug)]
pub struct PlayerTailParticleState {
    position: Vec3F,
    lifetime: Duration,
    impulse: Vec3F,
}

impl Default for PlayerTailParticleState {
    fn default() -> Self {
        Self {
            position: Vec3F::unit_x(),
            lifetime: Duration::from_millis(1500),
            impulse: -Vec3F::unit_x(),
        }
    }
}

impl PlayerTailParticleState {
    pub fn new(position: Vec3F, lifetime: Duration, impulse: Vec3F) -> Self {
        Self {
            position,
            lifetime,
            impulse,
        }
    }
}

type PlayerTailStateData<'a> = ReadStorage<'a, DrawableId>;

#[derive(Default, Debug)]
pub struct PlayerTailDelegate {
    id: Option<DrawableId>,
    material: Vec<Material>,
}
impl<'a> EntityDelegate<'a> for PlayerTailDelegate {
    type State = PlayerTailParticleState;
    type EntityResources = PlayerTailStateData<'a>;

    fn create<'b, F: Fn() -> MyBuilder<'a, 'b>>(
        &self,
        state: &Self::State,
        _resources: &mut Self::EntityResources,
        constructor: F,
    ) -> Vec<Entity> {
        if let Some(d_id) = &self.id {
            vec![constructor()
                .with(TransformComponent::new(
                    state.position,
                    Vec3F::new(0.5f32, 0.5f32, 0.5f32),
                    QuatF::zero(),
                ))
                .with(Particle {
                    lifetime: state.lifetime,
                })
                .with(RigidBody::new(state.impulse, -Vec3F::unit_y()))
                .with(d_id.clone())
                .with(self.material[rand_ind(0, self.material.len())].clone())
                .with(Gravity)
                .build()]
        } else {
            panic!("Tried to make a sprite but it hasn't been pre-initialized!");
        }
    }

    fn setup_delegate(&mut self, world: &mut World) {
        let mut renderer = world.write_resource::<Renderer>();
        let state = Sprite::new(SPARK_NAMES[0], true);
        let d_id = renderer.submit_model(state.mesh());
        self.id = Some(d_id);
        self.material = SPARK_NAMES
            .iter()
            .map(|tex_name| {
                let mut mat = state.material();
                let tex = Texture::from_file(tex_name);
                mat.diffuse_texture(tex);
                mat
            })
            .collect()
    }
}

const SPARK_NAMES: [&str; 3] = [
    "resources/flappy_bird/sparks/spark1.png",
    "resources/flappy_bird/sparks/spark3.png",
    "resources/flappy_bird/sparks/spark2.png",
];
