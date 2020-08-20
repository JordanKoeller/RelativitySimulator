use renderer::{Shader, IShader, Camera};
use mechanics::Player;
use stateful::Scene;

pub struct Renderer {
}

impl Renderer {
    pub fn render(&self, scene: &Scene) {
        for r in scene.get_renderables() {
            let s = r.shader();
            s.use_program();
            for (u_name, u_value) in scene.get_player().uniform_manager().get_all() {
                s.set_uniform(u_name, u_value);
            }
            r.render();
        }
    }

    pub fn new() -> Renderer {
        Renderer {}
    }
}