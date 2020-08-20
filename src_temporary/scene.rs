use cgmath::{perspective, Deg, Matrix4, Vector3};
use std::ffi::CStr;

use camera::Camera;
use drawable::{Drawable, Grid, Model, Skybox};
use physics::movable::Movable;
use player::Player;
use shader::Shader;
use shader_manager::ShaderManager;

pub struct Scene {
    model: Vec<Box<dyn Drawable>>,
    width: f32,
    height: f32,
}

impl Scene {
    pub fn new(models: Vec<Box<dyn Drawable>>, w: f32, h: f32) -> Scene {
        let c = models;
        Scene {
            model: c,
            width: w,
            height: h,
        }
    }

    pub fn grid_scene(shader_manager: &mut ShaderManager, w: f32, h: f32) -> (Scene, Player) {
        let shader = Shader::tesselation_pipeline(
            "shaders/tesselation/vs.glsl",
            "shaders/debug/wire_grid_fs.glsl",
            "shaders/tesselation/cs.glsl",
            "shaders/tesselation/es.glsl",
        );
        let model_matrix = Matrix4::from_scale(10.0);
        let grid = Grid::new(10, 3, model_matrix);
        let scene = Scene::new(vec![Box::new(grid)], w, h);

        let player = Player {
            position: Vector3::new(5.0, 15.0, -50.0),
            front: Vector3::new(0.0, 0.0, 1.0),
            ..Player::default()
        };
        shader_manager.add_shader("world".to_string(), shader);
        (scene, player)
    }

    pub fn colorbox_scene(shader_manager: &mut ShaderManager, w: f32, h: f32) -> (Scene, Player) {
        let shader = Shader::tesselation_pipeline(
            "shaders/tesselation/vs.glsl",
            "shaders/debug/color_box_fs.glsl",
            "shaders/tesselation/cs.glsl",
            "shaders/tesselation/es.glsl",
        );
        let seed_matrix = Matrix4::from_translation(Vector3::new(5.0, -5.0, 0.0)) *Matrix4::from_scale(20.0);
        let mut boxes = Vec::<Box<dyn Drawable>>::new();
        for i in 0..5 {
            let model_matrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, -100.0 * i as f32)) * seed_matrix;
            let bx = Box::new(Model::new("resources/objects/block/Grass_Block.obj", model_matrix)) as Box<dyn Drawable>;
            boxes.push(bx);
        }
        let scene = Scene::new(boxes, w, h);

        let player = Player {
            position: Vector3::new(80.0, 200.0, -80.0),
            front: Vector3::new(0.0, 0.0,-1.0),
            zoom: 80.0,
            ..Player::default()
        };
        shader_manager.add_shader("world".to_string(), shader);
        (scene, player)
    }

    pub fn city_scene(shader_manager: &mut ShaderManager, w: f32, h: f32) -> (Scene, Player) {
        let shader = Shader::tesselation_pipeline(
            "shaders/tesselation/vs.glsl",
            "shaders/tesselation/fs.glsl",
            "shaders/tesselation/cs.glsl",
            "shaders/tesselation/es.glsl",
        );
        let sky_shader = Shader::new("shaders/skybox/skybox.vs", "shaders/skybox/skybox.fs");
        let model = Matrix4::from_angle_x(cgmath::Rad::from(cgmath::Deg(0.0)));
        let model_matrix = Matrix4::from_scale(0.02) * model;
        let suit = Model::new("resources/objects/Camellia City/OBJ/Camellia_City.obj", model_matrix);
        shader_manager.add_shader("world".to_string(), shader);
        shader_manager.add_shader("skybox".to_string(), sky_shader);
        let skybox = Skybox::new([
            "resources/Skybox/right.jpg".to_string(),
            "resources/Skybox/left.jpg".to_string(),
            "resources/Skybox/top.jpg".to_string(),
            "resources/Skybox/bottom.jpg".to_string(),
            "resources/Skybox/front.jpg".to_string(),
            "resources/Skybox/back.jpg".to_string(),
        ]);
        let player = Player {
            position: Vector3::new(1.0, 5.0, -123.0),
            ..Player::default()
        };
        let scene = Scene::new(vec![Box::new(suit), Box::new(skybox), ], w, h);
        (scene, player)
    }

    pub fn draw(&self, player: &Player, shader_manager: &ShaderManager, lorentz_flag: i32) {
        for model in self.model.iter() {
            let shader = shader_manager.get_shader(model.shader_name());
            model.pre_draw(shader_manager);
            self.set_uniforms(player, shader, self.width, self.height, lorentz_flag);
            model.draw(shader_manager);
        }
    }

    fn set_uniforms(&self, player: &Player, shader: &Shader, width: f32, height: f32, lorentz_flag: i32) {
        let view = player.get_view_matrix();
        let frustum = Vector3::new(player.zoom() * width / height, player.zoom(), 0.0);
        let projection: Matrix4<f32> = perspective(Deg(player.zoom()), width / height, 0.1, 10000.0);
        shader.set_int(c_str!("lorentzFlag"), lorentz_flag);
        shader.set_mat4(c_str!("view"), &view);
        shader.set_mat4(c_str!("projection"), &projection);
        shader.set_vector3(c_str!("cameraPos"), &player.pos());
        shader.set_vector3(c_str!("cameraVelocity"), &player.beta_vector());
        shader.set_mat3(c_str!("changeOfBasis"), &player.velocity_basis_matrix());
        shader.set_mat3(c_str!("changeOfBasisInverse"), &player.velocity_inverse_basis_matrix());
        shader.set_float(c_str!("gamma"), player.gamma());
        shader.set_float(c_str!("beta"), player.beta());
        shader.set_vec3(c_str!("ambientLight"), 1.0, 1.0, 1.0);
        shader.set_vec3(c_str!("diffuseLight"), 0.0, 0.0, 0.0);
        shader.set_vec3(c_str!("specularLight"), 0.0, 0.0, 0.0);
        shader.set_vec3(c_str!("directionalLight"), 0.3, -1.0, 0.0);
        shader.set_float(c_str!("spacing"), 100.0);
        shader.set_vector3(c_str!("frustum"), &frustum);
        let mut view = view;
        view.w[0] = 0.0;
        view.w[1] = 0.0;
        view.w[2] = 0.0;
        shader.set_mat4(c_str!("skyboxView"), &view);
    }
}
