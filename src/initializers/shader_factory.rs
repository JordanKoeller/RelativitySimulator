use std::ffi::CString;
use std::ptr;
use std::str;

use gl;
use gl::types::*;

use initializers::shader_preprocessor::shader_preprocessor;
use initializers::{Factory, ShaderSpec};
use renderer::shader::{Shader};

pub struct ShaderFactory {}

impl Default for ShaderFactory {
    fn default() -> ShaderFactory {
        ShaderFactory {}
    }
}

impl ShaderFactory {
    // pub fn new_debug_program(&self, spec: Box<dyn ShaderSpec>) -> DebugShader {
    //     let mut components = Vec::new();
    //     let mut all_uniforms = std::collections::HashSet::new();
    //     for (comp_path, comp_type) in spec.shader_parts().iter() {
    //         let (body, uniforms) = shader_preprocessor(comp_path);
    //         let vert_shader = self.compile_shader(body, comp_type.clone());
    //         components.push(vert_shader);
    //         uniforms.iter().for_each(|e| {
    //             all_uniforms.insert(e.clone());
    //         });
    //     }
    //     let shader = self.link_program(components);
    //     DebugShader::new(shader, all_uniforms)
    // }

    pub fn new_program(&self, spec: Box<dyn ShaderSpec>) -> Shader {
        println!("Making shader");
        let mut components = Vec::new();
        let mut all_uniforms = std::collections::HashSet::new();
        for (comp_path, comp_type) in spec.shader_parts().iter() {
            let (body, uniforms) = shader_preprocessor(comp_path);
            let vert_shader = self.compile_shader(body, comp_type.clone());
            components.push(vert_shader);
            uniforms.iter().for_each(|e| {
                all_uniforms.insert(e.clone());
            });
        }
        self.link_program(components)
    }

    fn compile_shader(&self, shader_code: String, shader_type: GLenum) -> u32 {
        unsafe {
            let s = gl::CreateShader(shader_type);
            let c_code = CString::new(shader_code.as_bytes()).expect("Could not c string from shader body");
            gl::ShaderSource(s, 1, &c_code.as_ptr(), ptr::null());
            gl::CompileShader(s);
            let shader_type_string = self.get_type_string(shader_type);
            self.check_compile_errors(&s, &shader_type_string);
            s
        }
    }

    fn link_program(&self, shader_ids: Vec<u32>) -> Shader {
        unsafe {
            let shader_id = gl::CreateProgram();
            for script in shader_ids.iter() {
                gl::AttachShader(shader_id, script.clone());
            }
            gl::LinkProgram(shader_id);
            self.check_compile_errors(&shader_id, "PROGRAM");
            for script in shader_ids.iter() {
                gl::DeleteShader(script.clone());
            }
            Shader::new(shader_id)
        }
    }

    fn check_compile_errors(&self, shader: &u32, shader_type: &str) {
        unsafe {
            let mut success = gl::FALSE as GLint;
            let mut info_log: [u8; 2048] = [0; 2048];
            if shader_type != "PROGRAM" {
                gl::GetShaderiv(shader.clone(), gl::COMPILE_STATUS, &mut success);
                if success != gl::TRUE as GLint {
                    gl::GetShaderInfoLog(
                        shader.clone(),
                        1024,
                        ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );
                    println!(
                        "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                                -- --------------------------------------------------- -- ",
                        shader_type,
                        str::from_utf8(&info_log).expect("Encountered error on utf8 cast in SHADER check")
                    );
                    panic!("");
                }
            } else {
                gl::GetProgramiv(shader.clone(), gl::LINK_STATUS, &mut success);
                if success != gl::TRUE as GLint {
                    gl::GetProgramInfoLog(
                        shader.clone(),
                        1024,
                        ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );
                    println!(
                        "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                                -- --------------------------------------------------- -- ",
                        shader_type,
                        str::from_utf8(&info_log).expect("Encountered error on utf8 cast in PROGRAM check")
                    );
                    panic!("");
                }
            }
        }
    }

    fn get_type_string(&self, shader_type: GLenum) -> String {
        match shader_type {
            gl::VERTEX_SHADER => "VERTEX".to_string(),
            gl::FRAGMENT_SHADER => "FRAGMENT".to_string(),
            gl::TESS_EVALUATION_SHADER => "EVALUATION".to_string(),
            gl::TESS_CONTROL_SHADER => "CONTROL".to_string(),
            _ => panic!(format!("Could not convert {} to string.", shader_type)),
        }
    }
}

impl Factory for ShaderFactory {
    type Resource = Shader;
    type Spec = Box<dyn ShaderSpec>;
    fn new_resource(&self, spec: Self::Spec) -> Self::Resource {
        self.new_program(spec)
    }
}
