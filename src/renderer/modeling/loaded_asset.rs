use std::path::Path;

use tobj;

use renderer::{VertexArray, Drawable, Material, Textures, Texture, VertexBuffer, BufferLayout, AttributeType};

struct Mesh {
  vertex_array: VertexArray,
  textures: Textures,
  
}


pub struct Model {
  filename: String,
  meshes: Vec<Mesh>,
}


impl Model {
  pub fn new(path: &str) {
    // let directory = Path::new(path).parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
    let (meshVec, materials) = tobj::load_obj(path, true).expect(&format!("Failed to load file {}", path));
    let myMeshes = meshVec.into_iter().map(|mesh| {
      let mut atts: Vec<AttributeType> = vec![AttributeType::Float3];
      let mut buffer: Vec<f32> = Vec::new();
      for i in 0..mesh.positions.len()/3 {
        
      }

    });
  }
}