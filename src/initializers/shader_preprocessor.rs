use renderer::uniform::UniformType;

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::str;

use regex::Regex;

lazy_static! {
    static ref INCLUDE_MATCHER: Regex = Regex::new("(?m)^\\s*#include \"([a-z./_]+)\"\\s*$").unwrap();
    static ref UNIFORM_MATCHER: Regex = Regex::new("(?m)^\\s*uniform ([A-Za-z_0-9]+) ([A-Za-z_0-9]+);\\s*$").unwrap();
}

pub fn shader_preprocessor(shader_path: &str) -> (String, HashSet<(String, UniformType)>) {
    let mut included_files = HashSet::new();
    let mut uniforms = HashSet::new();
    let ret = preprocessor_helper(shader_path, &mut included_files);
    for uniform in UNIFORM_MATCHER.captures_iter(&ret) {
        let name = String::from(&uniform[2]);
        match &uniform[1] {
            "vec3" => {
                uniforms.insert((name, UniformType::Vec3));
            }
            // "vec4" => uniforms.insert((name, UniformType::Vec4)),
            "mat4" => {
                uniforms.insert((name, UniformType::Mat4));
            }
            "mat3" => {
                uniforms.insert((name, UniformType::Mat3));
            }
            "sampler2D" => {
                uniforms.insert((name, UniformType::Int));
            }
            "sampler3D" => {
                uniforms.insert((name, UniformType::Int));
            }
            "float" => {
                uniforms.insert((name, UniformType::Float));
            }
            "int" => {
                uniforms.insert((name, UniformType::Int));
            }
            "bool" => {
                uniforms.insert((name, UniformType::Bool));
            }
            _ => panic!(
                "Shader contains unrecognized uniform 'uniform {} {};",
                &uniform[1], &uniform[2]
            ),
        };
    }
    (ret, uniforms)
}

fn preprocessor_helper(shader_path: &str, included_files: &mut HashSet<String>) -> String {
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
        let inc_body = preprocessor_helper(&inc_file, included_files);
        ret = ret.replace(&import_file[0], &inc_body); // Will fail because ret is different from shader_body
    }
    ret
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_construct_simple_shader() {
        let (shader_body, uniforms) = shader_preprocessor("test_resources/simple_shader.fs");
        let expected_body = "#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;
out vec2 TexCoords;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
void main()
{
    TexCoords = aTexCoords;
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}";
        let expected_uniforms = vec![
            ("model".to_string(), UniformType::Mat4),
            ("view".to_string(), UniformType::Mat4),
            ("projection".to_string(), UniformType::Mat4),
        ];
        let expected_uniforms: HashSet<(String, UniformType)> = expected_uniforms.iter().cloned().collect();
        assert_eq!(expected_body, shader_body);
        assert_eq!(uniforms.len(), expected_uniforms.len());
        assert_eq!(uniforms, expected_uniforms);
    }

    #[test]
    fn can_construct_complex_shader() {
        let (shader_body, uniforms) = shader_preprocessor("test_resources/complex_shader.fs");
        let expected_body = "#version 410 core
uniform float beta;
uniform float gamma;
uniform int lorentzFlag;
uniform vec3 cameraPos;
uniform vec3 frustum;
uniform mat3 changeOfBasis;
uniform mat3 changeOfBasisInverse;
// uniform mat3 commented_out_uniform;
vec3 lorentzTransform(vec3 pos)
{
vec3 refFramePos = changeOfBasis * (pos - cameraPos);
vec3 transformed = vec3(refFramePos.x/gamma, refFramePos.y, refFramePos.z);
return changeOfBasisInverse * transformed + cameraPos;
}
uniform mat4 view;
uniform mat4 projection;
vec3 interpolate3D(vec3 v0, vec3 v1, vec3 v2)
{
return vec3(gl_TessCoord.x) * v0 + vec3(gl_TessCoord.y) * v1 + vec3(gl_TessCoord.z) * v2;
}";
        let expected_uniforms = vec![
            ("beta".to_string(), UniformType::Float),
            ("gamma".to_string(), UniformType::Float),
            ("lorentzFlag".to_string(), UniformType::Int),
            ("cameraPos".to_string(), UniformType::Vec3),
            ("frustum".to_string(), UniformType::Vec3),
            ("changeOfBasis".to_string(), UniformType::Mat3),
            ("changeOfBasisInverse".to_string(), UniformType::Mat3),
            ("view".to_string(), UniformType::Mat4),
            ("projection".to_string(), UniformType::Mat4),
        ];
        let expected_uniforms: HashSet<(String, UniformType)> = expected_uniforms.iter().cloned().collect();
        assert_eq!(expected_body, shader_body);
        assert_eq!(uniforms.len(), 9);
        assert_eq!(uniforms, expected_uniforms);
    }
}
