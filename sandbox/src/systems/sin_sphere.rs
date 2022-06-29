use cgmath::prelude::*;
use specs::prelude::*;
use specs::SystemData;
use std::f32::consts::PI;

use engine::ecs::{MonoBehavior, SystemUtilities, WorldProxy, PrefabBuilder};
use engine::gui::{widgets::*, ControlPanelBuilder, SystemDebugger};
use engine::utils::{Vec3F, Color, Vec2F};
use engine::graphics::MeshComponent;
use engine::prefab::{Sphere, SphereState};
use engine::graphics::Vertex;

#[derive(SystemData)]
pub struct SinSphereSystemData<'a> {
    pub meshes: WriteStorage<'a, MeshComponent>,
}

#[derive(Default)]
pub struct SinSphere {
    entity: Option<Entity>,
}

impl<'a> MonoBehavior<'a> for SinSphere {
    type SystemData = SinSphereSystemData<'a>;

    fn run(&mut self, api: SystemUtilities<'a>, mut s: Self::SystemData) {
        if let Some(entity) = self.entity {
            let panel = self.get_panel(&api);
            let amplitude = panel.get_float("Amplitude");
            let freq = panel.get_float("Frequency");
            let phase = panel.get_float("Phase");
            let mesh_component = s.meshes.get_mut(entity).unwrap();
            let mut mesh = api.get_mesh_mut(&mut mesh_component.vertex_array_id).unwrap();
            let mut mesh_view = mesh.as_view::<Vertex>();
            for i in 0..mesh_view.len() {
                let mut vec = mesh_view.set(i);
                let rise = vec.normal.y;
                let run = Vec2F::new(vec.normal.x, vec.normal.z).magnitude();
                let angle = rise.atan2(run);
                let height = 0.5f32 + amplitude * (freq*angle+phase).sin();
                let delta_vec = vec.normal * height;
                vec.position = vec.normal * 0.5 + delta_vec;
            }
        }
    }

    fn setup(&mut self, mut world: WorldProxy) {
        Self::SystemData::setup(&mut world);
        self.register_debugger(&world);
        let mut sphere_builder = Sphere::default();
        let sphere_state = SphereState::new(
            3f32,
            Vec3F::new(0f32, 0f32, 0f32),
            Color::new(1f32, 0.3f32, 0.3f32),
            "resources/earth/2k_earth_daymap.jpg",
            "resources/earth/2k_earth_specular_map.png",
            "resources/earth/2k_earth_normal_map.png",
            64,
        );
        let sphere = sphere_builder.build(&world.utilities(), sphere_state);
        self.entity = Some(sphere);
    }
}

impl<'a> SystemDebugger<'a> for SinSphere {
    fn create_panel(&self) -> ControlPanelBuilder {
        ControlPanelBuilder::default()
            .with_title("Sin Sphere")
            .push_line("Amplitude", InputFloat::new_with_limits("Amplitude", 0.1f32, 0f32, 1f32))
            .push_line("Frequency", InputFloat::new_with_limits("Frequency", PI / 2f32, 0f32, 50f32))
            .push_line("Phase", InputFloat::new_with_limits("Phase", 0f32, 0f32, 50f32))
    }
}