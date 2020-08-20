use gl;

pub trait ShaderSpec {
    fn shader_parts(&self) -> Vec<(String, gl::types::GLenum)>; // Tuples of pairs of path and shader type
}

pub struct TessShaderSpec {
    vert_path: String,
    ctrl_path: String,
    eval_path: String,
    frag_path: String,
}

impl TessShaderSpec {
    pub fn new(vert: &str, ctrl: &str, eval: &str, frag: &str) -> TessShaderSpec {
        TessShaderSpec {
            vert_path: vert.to_string(),
            ctrl_path: ctrl.to_string(),
            eval_path: eval.to_string(),
            frag_path: frag.to_string()
        }
    }
}

impl ShaderSpec for TessShaderSpec {
    fn shader_parts(&self) -> Vec<(String, gl::types::GLenum)> {
        vec![
            (self.vert_path.clone(), gl::VERTEX_SHADER),
            (self.ctrl_path.clone(), gl::TESS_CONTROL_SHADER),
            (self.eval_path.clone(), gl::TESS_EVALUATION_SHADER),
            (self.frag_path.clone(), gl::FRAGMENT_SHADER),
            ]
    }
}


pub struct NormalShaderSpec {
    vert_path: String,
    frag_path: String,
}

impl NormalShaderSpec {
    pub fn new(vert: &str, frag: &str) -> NormalShaderSpec {
        NormalShaderSpec {
            vert_path: vert.to_string(),
            frag_path: frag.to_string()
        }
    }
}

impl ShaderSpec for NormalShaderSpec {
    fn shader_parts(&self) -> Vec<(String, gl::types::GLenum)> {
        vec![
            (self.vert_path.clone(), gl::VERTEX_SHADER),
            (self.frag_path.clone(), gl::FRAGMENT_SHADER),
            ]
    }
}