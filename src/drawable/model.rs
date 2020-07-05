use cgmath::{vec2, vec3, Matrix4};
use gl;
use image;
use image::DynamicImage::*;
use image::GenericImage;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::path::Path;
use std::vec;
use tobj;

use drawable::mesh::{Mesh, Texture, Vertex};
use drawable::Drawable;
use shader_manager::ShaderManager;

pub struct Model {
  /*  Model Data */
  pub meshes: vec::Vec<Mesh>,
  pub textures_loaded: vec::Vec<Texture>, // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
  shader_name: String,
  model_matrix: Matrix4<f32>,
  directory: String,
}

impl Model {
  /// constructor, expects a filepath to a 3D model.
  pub fn new(path: &str, matrix: Matrix4<f32>, shader_name: String) -> Model {
    let mut model = Model {
      meshes: Vec::default(),
      textures_loaded: Vec::default(),
      model_matrix: matrix,
      shader_name: shader_name,
      directory: String::default(),
    };
    model.load_model(path);
    model
  }

  // loads a model from file and stores the resulting meshes in the meshes vector.
  fn load_model(&mut self, path: &str) {
    let path = Path::new(path);

    // retrieve the directory path of the filepath
    self.directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
    let obj = tobj::load_obj(path);

    let (models, materials) = obj.unwrap();
    for model in models {
      let mesh = &model.mesh;
      let num_vertices = mesh.positions.len() / 3;

      // data to fill
      let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
      let indices: Vec<u32> = mesh.indices.clone();

      let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
      for i in 0..num_vertices {
        vertices.push(Vertex {
          position: vec3(p[i * 3], p[i * 3 + 1], p[i * 3 + 2]),
          normal: vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2]),
          tex_coords: vec2(t[i * 2], t[i * 2 + 1]),
          ..Vertex::default()
        })
      }

      // process material
      let mut textures = Vec::new();
      if let Some(material_id) = mesh.material_id {
        let material = &materials[material_id];

        // 1. diffuse map
        if !material.diffuse_texture.is_empty() {
          let texture = self.load_material_texture(&material.diffuse_texture, "texture_diffuse");
          textures.push(texture);
        }
        // 2. specular map
        if !material.specular_texture.is_empty() {
          let texture = self.load_material_texture(&material.specular_texture, "texture_specular");
          textures.push(texture);
        }
        // 3. normal map
        if !material.normal_texture.is_empty() {
          let texture = self.load_material_texture(&material.normal_texture, "texture_normal");
          textures.push(texture);
        }
        // NOTE: no height maps
      }

      self.meshes.push(Mesh::new(vertices, indices, textures));
    }
  }

  fn load_material_texture(&mut self, path: &str, type_name: &str) -> Texture {
    {
      let texture = self.textures_loaded.iter().find(|t| t.path == path);
      if let Some(texture) = texture {
        return texture.clone();
      }
    }

    let texture = Texture {
      id: texture_from_file(path, &self.directory),
      type_: type_name.into(),
      path: path.into(),
    };
    self.textures_loaded.push(texture.clone());
    texture
  }
}

impl Drawable for Model {
  fn shader_name(&self) -> String {
    "world".to_string()
  }

  fn draw(&self, shader: &ShaderManager) {
    let s = shader.get_shader(self.shader_name());
    s.set_mat4(c_str!("model"), &self.model_matrix);
    for mesh in &self.meshes {
      unsafe {
        mesh.draw(s);
      }
    }
  }
}

fn texture_from_file(path: &str, directory: &str) -> u32 {
  unsafe {
    let filename = format!("{}/{}", directory, path);

    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);

    let img = image::open(&Path::new(&filename)).expect("Texture failed to load");
    let img = img.flipv();
    let format = match img {
      ImageLuma8(_) => gl::RED,
      ImageLumaA8(_) => gl::RG,
      ImageRgb8(_) => gl::RGB,
      ImageRgba8(_) => gl::RGBA,
    };

    let data = img.raw_pixels();

    gl::BindTexture(gl::TEXTURE_2D, texture_id);
    gl::TexImage2D(
      gl::TEXTURE_2D,
      0,
      format as i32,
      img.width() as i32,
      img.height() as i32,
      0,
      format,
      gl::UNSIGNED_BYTE,
      &data[0] as *const u8 as *const c_void,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    texture_id
  }
}
