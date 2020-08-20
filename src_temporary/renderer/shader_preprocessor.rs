use renderer::uniform::UniformType;

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::str;

use regex::Regex;

const INCLUDE_MATCHER: Regex = Regex::new("#include \"([a-z./]+)\"").unwrap();
const UNIFORM_MATCHER: Regex = Regex::new("uniform ([A-Za-z_0-9]+) ([A-Za-z_0-9]+);").unwrap();

pub fn shader_preprocessor(shader_path: &str) -> (String, HashSet<(String, UniformType)>) {
    let mut included_files = HashSet::new();
    let mut uniforms = HashSet::new();
    let ret = preprocessor_helper(shader_path, &mut included_files);
    for uniform in UNIFORM_MATCHER.captures_iter(&ret) {
        let name = String::from(&uniform[2]);
        match &uniform[1] {
            "vec3" => {uniforms.insert((name, UniformType::Vec3));},
            // "vec4" => uniforms.insert((name, UniformType::Vec4)),
            "mat4" => {uniforms.insert((name, UniformType::Mat4));},
            "mat3" => {uniforms.insert((name, UniformType::Mat3));},
            "mat3" => {uniforms.insert((name, UniformType::Mat3));},
            "sampler2D" => {uniforms.insert((name, UniformType::UInt));},
            "sampler3D" => {uniforms.insert((name, UniformType::UInt));},
            "float" => {uniforms.insert((name, UniformType::Float));},
            "int" => {uniforms.insert((name, UniformType::Int));},
            "bool" => {uniforms.insert((name, UniformType::Bool));},
            _ => panic!("Shader contains unrecognized uniform 'uniform {} {};", &uniform[1], &uniform[2])
        };
    }
    (ret, uniforms)
}

fn preprocessor_helper(shader_path: &str, included_files: &mut HashSet<String>) -> String {
    // Adds a file path to the included_files path
    //   Returns the file
    let mut shader_file = File::open(shader_path).unwrap_or_else(|_| panic!("Failed to open {}", shader_path));
    let mut shader_body = String::new();
    shader_file
        .read_to_string(&mut shader_body)
        .expect("Failed to read shader body");
    let mut ret = shader_body.clone();
    for import_file in INCLUDE_MATCHER.captures_iter(&shader_body) {
        let inc_file = String::from(&import_file[1]);
        included_files.insert(inc_file.clone());
        let inc_body = preprocessor_helper(&inc_file, included_files);
        ret = ret.replace(&import_file[0], &inc_body); // Will fail because ret is different from shader_body
    }
    ret
}
