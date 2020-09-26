// use renderer::uniform::UniformType;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::str;

use regex::Regex;

use renderer::platform::{Shader, UniformType};

lazy_static! {
  static ref INCLUDE_MATCHER: Regex = Regex::new("(?m)^\\s*#include \"([a-z./_]+)\"\\s*$").unwrap();
  static ref UNIFORM_MATCHER: Regex = Regex::new("(?m)^\\s*uniform ([A-Za-z_0-9]+) ([A-Za-z_0-9]+);\\s*$").unwrap();
}

pub fn file_includer(shader_path: &str) -> String {
  // Reads for "# include" and pastes in the necessary files.
  let mut included_files = HashSet::new();
  let ret = file_includer_helper(shader_path, &mut included_files);
  ret
}

fn file_includer_helper(shader_path: &str, included_files: &mut HashSet<String>) -> String {
  // Adds a file path to the included_files path
  //   Returns the file
  let mut shader_file = File::open(shader_path).expect(&format!("Failed to open {}", shader_path));
  let mut shader_body = String::new();
  shader_file
    .read_to_string(&mut shader_body)
    .expect("Failed to read shader body");
  let mut ret = shader_body.clone();
  for import_file in INCLUDE_MATCHER.captures_iter(&shader_body) {
    let inc_file = String::from(&import_file[1]);
    included_files.insert(inc_file.clone());
    let inc_body = file_includer_helper(&inc_file, included_files);
    ret = ret.replace(&import_file[0], &inc_body); // Will fail because ret is different from shader_body
  }
  ret
}

fn identify_uniforms(shader: &Shader) -> HashMap<String, UniformType> {
  let mut uniforms = HashMap::new();
  for uniform in UNIFORM_MATCHER.captures_iter(&shader.program_source) {
    let name = String::from(&uniform[2]);
    match &uniform[1] {
      "vec3" => uniforms.insert(name, UniformType::Vec3),
      "mat4" => uniforms.insert(name, UniformType::Mat4),
      "mat3" => uniforms.insert(name, UniformType::Mat3),
      "sampler2D" => uniforms.insert(name, UniformType::Int),
      "sampler3D" => uniforms.insert(name, UniformType::Int),
      "float" => uniforms.insert(name, UniformType::Float),
      "int" => uniforms.insert(name, UniformType::Int),
      "bool" => uniforms.insert(name, UniformType::Bool),
      "samplerCube" => uniforms.insert(name, UniformType::Int),
      _ => panic!(
        "Shader contains unrecognized uniform 'uniform {} {};",
        &uniform[1], &uniform[2]
      ),
    };
  }
  uniforms
}
