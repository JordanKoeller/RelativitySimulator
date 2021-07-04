
use tobj;

use renderer::{
  AttributeType, BufferLayout, IndexBuffer, Texture, VertexArray, VertexBuffer, WHITE_TEXTURE,
};
use utils::*;

use ecs::Material;

// struct Mesh {
//   vertex_array: VertexArray,
//   material: Material,
// }

#[allow(dead_code)]
pub struct Model {
  filename: String,
  shader: String,
  pub meshes: Vec<MeshComponent>,
}

impl Model {
  #[allow(dead_code)]
  pub fn new(path: &str, shader: &str) -> Model {
    // let directory = Path::new(path).parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
    let (model_vec, materials) = tobj::load_obj(path, true).expect(&format!("Failed to load file {}", path));
    let last_path_divider = path.rfind("/").expect("Could not remove filename from path");
    let dirpath = path[..last_path_divider].to_string();
    let my_materials = get_materials(&materials, dirpath);
    let meshes = model_vec
      .into_iter()
      .map(move |model| {
        let vert_arr = get_vertex_array(&model.mesh);
        if let Some(mat_id) = model.mesh.material_id {
          Box::from(DrawableState::new_textured(vert_arr, my_materials[mat_id].clone()))
        } else {
          Box::from(DrawableState::new_textured(vert_arr, Material::new()))
        }
      })
      .collect();
    Model {
      filename: path.to_string(),
      shader: shader.to_string(),
      meshes,
    }
  }
}

fn get_vertex_array(mesh: &tobj::Mesh) -> VertexArray {
  let mut atts: Vec<AttributeType> = vec![AttributeType::Float3];
  if mesh.normals.len() > 0 {
    atts.push(AttributeType::Float3)
  }
  if mesh.texcoords.len() > 0 {
    atts.push(AttributeType::Float2)
  }
  // Copy over the data into my buffer
  let mut buffer: Vec<f32> = Vec::with_capacity(mesh.positions.len() * 3);

  for i in 0..mesh.positions.len() / 3 {
    buffer.push(mesh.positions[i * 3]);
    buffer.push(mesh.positions[i * 3 + 1]);
    buffer.push(mesh.positions[i * 3 + 2]);
    if mesh.normals.len() > 0 {
      buffer.push(mesh.normals[i * 3]);
      buffer.push(mesh.normals[i * 3 + 1]);
      buffer.push(mesh.normals[i * 3 + 2]);
    }
    if mesh.texcoords.len() > 0 {
      buffer.push(mesh.texcoords[i * 2]);
      buffer.push(mesh.texcoords[i * 2 + 1]);
    }
  }
  let buff = VertexBuffer::create(buffer, BufferLayout::new(atts));
  let inds = IndexBuffer::create(mesh.indices.clone());
  VertexArray::new(vec![buff], inds)
}

fn as_vec(v: &[f32; 3]) -> Vec3F {
  Vec3F::new(v[0], v[1], v[2])
}

fn tex_or_default(dirpath: &str, relpath: &str) -> Texture {
  if relpath != "" {
    Texture::from_file(&format!("{}/{}", dirpath, relpath))
  } else {
    WHITE_TEXTURE.clone()
  }
}

fn get_materials(mats: &Vec<tobj::Material>, dirpath: String) -> Vec<Material> {
  mats
    .into_iter()
    .map(|mat| {
      let mut ret = Material::new();
      ret.ambient(as_vec(&mat.ambient));
      ret.diffuse(as_vec(&mat.diffuse));
      ret.specular(as_vec(&mat.specular));
      ret.diffuse(as_vec(&mat.diffuse));
      ret.shininess(mat.shininess);
      ret.dissolve(mat.dissolve);
      ret.optical_density(mat.optical_density);
      ret.ambient_texture(tex_or_default(&dirpath, &mat.ambient_texture));
      ret.diffuse_texture(tex_or_default(&dirpath, &mat.diffuse_texture));
      ret.specular_texture(tex_or_default(&dirpath, &mat.specular_texture));
      ret.normal_texture(tex_or_default(&dirpath, &mat.normal_texture));
      ret.shininess_texture(tex_or_default(&dirpath, &mat.shininess_texture));
      ret.dissolve_texture(tex_or_default(&dirpath, &mat.dissolve_texture));
      ret
    })
    .collect()
}

// impl Drawable for Mesh {
//   fn vertex_array(&self) -> &VertexArray {
//     &self.vertex_array
//   }
//   fn material(&self) -> &Material {
//     &self.material
//   }
// }
