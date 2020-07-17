use scene::Scene;
use cgmath::{perspective, Deg, Matrix4, Vector3};
use std::ffi::CStr;

use camera::Camera;
use drawable::{Drawable, Grid, Model, Skybox, Cube};
use physics::movable::Movable;
use player::Player;
use shader::Shader;
use shader_manager::ShaderManager;
use drawable::SimpleBuilding;

pub fn procedure_scene(shader_manager: &mut ShaderManager, w: f32, h: f32) -> (Scene, Player) {
    let cube_shader = Shader::new(
        "shaders/1.model_loading.vs",
        "shaders/1.model_loading.fs",
    );
    let seed_matrix = Matrix4::from_translation(Vector3::new(0.0, -50.0, 0.0)) * Matrix4::from_scale(1.0);
    let model_matrix = Matrix4::from_nonuniform_scale(1000.0, 1.0, 1000.0) * seed_matrix;
    // let model_matrix = Matrix4::from_translation(Vector3::new(10.0, -8.0, 10.0)) * seed_matrix;
    // let model_matrix = Matrix4::from_scale(10.0);
    let floor = Cube::with_texture("resources/textures/checkerboard.png", model_matrix);

    let shader = Shader::tesselation_pipeline(
        "shaders/tesselation/vs.glsl",
        "shaders/debug/color_box_fs.glsl",
        "shaders/tesselation/cs.glsl",
        "shaders/tesselation/es.glsl",
    );
    let mut boxes = Vec::<Box<dyn Drawable>>::new();
    for i in 0..5 {
        let model_matrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, -100.0 * i as f32)) * seed_matrix;
        let bx = Box::new(Model::new("resources/objects/block/Grass_Block.obj", model_matrix)) as Box<dyn Drawable>;
        boxes.push(bx);
    }
    boxes.push(Box::new(floor));

    let scene = Scene::new(boxes, w, h);

    let player = Player {
        position: Vector3::new(80.0, 0.0, -80.0),
        front: Vector3::new(0.0, 0.0,-1.0),
        zoom: 80.0,
        ..Player::default()
    };
    shader_manager.add_shader("world".to_string(), shader);
    shader_manager.add_shader("cube".to_string(), cube_shader);
    (scene, player)
}