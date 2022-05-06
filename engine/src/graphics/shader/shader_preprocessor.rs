use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use cgmath::prelude::*;
use regex::Regex;

use crate::graphics::Uniform;
#[derive(Clone, Eq, PartialEq)]
pub enum ShaderStep {
    VertexShader(String),
    FragmentShader(String),
    // ComputeShader(String),
    GeometryShader(String),
    TessControlShader(String),
    TessEvalShader(String),
}

impl ShaderStep {
    pub fn typestring(&self) -> String {
        match self {
            ShaderStep::VertexShader(_) => "VERTEX_SHADER".to_string(),
            ShaderStep::FragmentShader(_) => "FRAGMENT_SHADER".to_string(),
            ShaderStep::TessControlShader(_) => "TESS_CONTROL_SHADER".to_string(),
            ShaderStep::TessEvalShader(_) => "TESS_EVALUATION_SHADER".to_string(),
            ShaderStep::GeometryShader(_) => "GEOMETRY_SHADER".to_string(),
        }
    }

    pub fn text(&self) -> &str {
        match self {
            ShaderStep::VertexShader(s) => s,
            ShaderStep::FragmentShader(s) => s,
            ShaderStep::TessControlShader(s) => s,
            ShaderStep::TessEvalShader(s) => s,
            ShaderStep::GeometryShader(s) => s,
        }
    }

    pub fn gl_enum(&self) -> gl::types::GLenum {
        match self {
            ShaderStep::FragmentShader(_) => gl::FRAGMENT_SHADER,
            ShaderStep::VertexShader(_) => gl::VERTEX_SHADER,
            ShaderStep::TessControlShader(_) => gl::TESS_CONTROL_SHADER,
            ShaderStep::TessEvalShader(_) => gl::TESS_EVALUATION_SHADER,
            ShaderStep::GeometryShader(_) => gl::GEOMETRY_SHADER,
        }
    }
}

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

pub fn decompress(body: String) -> Vec<ShaderStep> {
    body.split("#shader ")
        .filter(|s| {
            if let Some(first_line) = s.lines().next() {
                if SHADER_OPTIONS.iter().any(|&e| e == first_line) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .map(|s| {
            let mut lines: Vec<&str> = s.lines().collect();
            let first_line = lines.remove(0);
            match first_line {
                "vertex" => ShaderStep::VertexShader(lines.join("\n")),
                "tesscontrol" => ShaderStep::TessControlShader(lines.join("\n")),
                "tesseval" => ShaderStep::TessEvalShader(lines.join("\n")),
                "fragment" => ShaderStep::FragmentShader(lines.join("\n")),
                "geometry" => ShaderStep::GeometryShader(lines.join("\n")),
                _ => panic!("Could not determine shader type from label"),
            }
        })
        .collect()
}

pub fn get_element_type(steps: &Vec<ShaderStep>) -> gl::types::GLenum {
    if steps.iter().any(|s| match s {
        ShaderStep::TessControlShader(_) => true,
        ShaderStep::TessEvalShader(_) => true,
        _ => false,
    }) {
        gl::PATCHES
    } else {
        gl::TRIANGLES
    }
}

pub fn compile_program(steps: Vec<ShaderStep>) -> u32 {
    unsafe {
        let program = gl::CreateProgram();
        for step in steps.into_iter() {
            compile_shader(&program, step);
        }
        gl::LinkProgram(program);
        // Error checking
        let mut err_log = Vec::with_capacity(1024);
        let mut err_code = 0;
        err_log.set_len(1024);
        for i in 0..1024 {
            err_log[i] = 0;
        }
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut err_code);
        if err_code != gl::TRUE as gl::types::GLint {
            gl::GetProgramInfoLog(
                program,
                1024,
                ptr::null_mut(),
                err_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            let value = str::from_utf8(&err_log);
            if value.is_ok() {
                println!(
                    "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                    value.unwrap()
                );
            } else {
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED and error message had error\n{}", value.err().unwrap());
            }
        }
        program
    }
}
unsafe fn compile_shader(program: &u32, shader: ShaderStep) {
    let shader_c_code = CString::new(shader.text().as_bytes()).unwrap();
    let shader_id = gl::CreateShader(shader.gl_enum());
    gl::ShaderSource(shader_id, 1, &shader_c_code.as_ptr(), ptr::null());
    gl::CompileShader(shader_id);
    let mut err_log = Vec::with_capacity(2048);
    let mut err_code = 0;
    err_log.set_len(2048);
    for i in 0..2048 {
        err_log[i] = 0;
    }
    gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut err_code);
    if err_code != gl::TRUE as gl::types::GLint {
        gl::GetShaderInfoLog(
            shader_id,
            2047,
            ptr::null_mut(),
            err_log.as_mut_ptr() as *mut gl::types::GLchar,
        );
        let value = str::from_utf8(&err_log);
        if value.is_ok() {
            println!(
                "ERROR::SHADER::{}::COMPILATION_FAILED\n{}",
                shader.typestring(),
                value.unwrap()
            );
        } else {
            println!("ERROR::SHADER::{}::COMPILATION_FAILED and error message had error\n{}", shader.typestring(), value.err().unwrap());
        }
    }
    gl::AttachShader(*program, shader_id);
    gl::DeleteShader(shader_id);
}

const SHADER_OPTIONS: [&str; 5] = ["vertex", "fragment", "tesseval", "tesscontrol", "geometry"];

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing::get_context;

    static TEST_SHADER_FILE: &str = "test_resources/complex_shader.fs"; // Junk file for testing parts
    static TEST_VALID_SHADER_FILE: &str = "test_resources/simple_shader.fs"; // A valid shader program

    #[test]
    fn test_file_includer_builds_full_shader_file() {
        let shader_body: String = file_includer(TEST_SHADER_FILE);
        let shader_lines: Vec<&str> = shader_body.split("\n").collect();
        assert_eq!(shader_lines.len(), 33);
    }

    #[test]
    fn test_preprocessor_breaks_shader_steps_into_programs() {
        let shader_body: String = file_includer(TEST_SHADER_FILE);
        let steps = decompress(shader_body);
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0].typestring(), "VERTEX_SHADER".to_string());
        assert_eq!(steps[1].typestring(), "FRAGMENT_SHADER".to_string());
    }

    #[test]
    fn test_preprocessor_gets_correct_element_type() {
        let patch_steps = vec![
            ShaderStep::VertexShader("Some Vertex".to_string()),
            ShaderStep::TessControlShader("Some Control".to_string()),
            ShaderStep::TessEvalShader("Some Eval".to_string()),
            ShaderStep::FragmentShader("Some Fragment".to_string()),
        ];

        let triangle_steps = vec![
            ShaderStep::VertexShader("Some Vertex".to_string()),
            ShaderStep::FragmentShader("Some Fragment".to_string()),
        ];
        assert_eq!(get_element_type(&patch_steps), gl::PATCHES);
        assert_eq!(get_element_type(&triangle_steps), gl::TRIANGLES);
    }

    #[test]
    fn test_shader_compiler() {
        let _ctx = get_context();
        let shader_body: String = file_includer(TEST_VALID_SHADER_FILE);
        let steps = decompress(shader_body);
        let _id = compile_program(steps);
        assert_eq!(1, 1);
    }
}
