use super::Shader;

type ShaderID = usize;

pub struct ShaderLibrary {
  lookup: std::collections::HashMap<String, usize>,
  shaders: Vec<Shader>,
}

impl Default for ShaderLibrary {
  fn default() -> ShaderLibrary {
    ShaderLibrary {
      lookup: std::collections::HashMap::new(),
      shaders: Vec::new()
    }
  }
}

impl ShaderLibrary {
  pub fn add(&mut self, shader: Shader) -> ShaderID {
    let name = shader.name.clone();
    let ind = self.shaders.len();
    self.lookup.insert(name, ind);
    self.shaders.push(shader);
    ind
  }

  pub fn get(&self, id: &str) -> &Shader {
    &self.shaders[*self.lookup.get(id).unwrap()]
  }
}