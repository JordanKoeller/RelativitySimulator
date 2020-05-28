use shader::Shader;
use std::collections::HashMap;

pub struct ShaderManager {
  shader_map: HashMap<String, Shader>
}

impl ShaderManager {

  pub fn new() -> ShaderManager {
    ShaderManager {
      shader_map: HashMap::new()
    }
  }
  pub fn get_shader(&self, name: String) -> &Shader {
    self.shader_map.get(&name).expect("Shader not found")
  }

  pub fn add_shader(&mut self, name: String, shader: Shader) {
    self.shader_map.insert(name, shader);
  }
}
