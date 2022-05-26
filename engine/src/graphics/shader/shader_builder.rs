use crate::datastructures::RegistryItem;
use crate::utils::{ReadAssetRef, RwAssetRef};
use gl;

use super::shader_delegates::{
    DepthFuncShaderBinder, ShaderBinder, ShaderDepthFunction, StdShaderBinder, UniformSlots,
};
use super::shader_preprocessor;
use super::Shader;
use super::ShaderId;

pub struct ShaderBuilder {
    filename: Option<String>,
    depth_function: gl::types::GLenum,
    shader_id: RwAssetRef<u32>,
}

impl Default for ShaderBuilder {
    fn default() -> Self {
        Self {
            filename: None,
            shader_id: RwAssetRef::new(std::u32::MAX),
            depth_function: gl::LESS,
        }
    }
}

impl ShaderBuilder {
    pub fn with_source_file(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_string());
        self
    }

    pub fn with_depth_function(mut self, depth_func: ShaderDepthFunction) -> Self {
        self.depth_function = depth_func.get_gl_enum();
        self
    }
}

impl RegistryItem for ShaderBuilder {
    type K = ShaderId;
    type V = Shader;

    fn key(&self) -> Self::K {
        ShaderId::new(self.shader_id.ro_ref())
    }

    fn is_buildable(&self) -> bool {
        self.filename.is_some()
    }

    fn build(mut self) -> Shader {
        let shader_body = shader_preprocessor::file_includer(&self.filename.unwrap());
        let binder: Box<dyn ShaderBinder> = if self.depth_function == gl::LESS {
            Box::from(StdShaderBinder)
        } else {
            Box::from(DepthFuncShaderBinder::new(self.depth_function))
        };
        let shader_steps = shader_preprocessor::decompress(shader_body);
        let element_type = shader_preprocessor::get_element_type(&shader_steps);
        let program_id = shader_preprocessor::compile_program(shader_steps);
        let uniform_slots = UniformSlots::default();
        self.shader_id.set(program_id);
        Shader::new(binder, self.shader_id, element_type, uniform_slots)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing::get_context;

    const SHADER_FILE: &str = "test_resources/simple_shader.fs";

    #[test]
    fn builder_is_buildable_if_file_set() {
        let builder = ShaderBuilder::default();
        assert_eq!(builder.is_buildable(), false);
        let builder = builder.with_source_file(SHADER_FILE);
        assert_eq!(builder.is_buildable(), true);
    }

    #[test]
    fn builder_builds_a_shader() {
        let _ctx = get_context();
        let builder = ShaderBuilder::default().with_source_file(SHADER_FILE);
        let shader_id = builder.key();
        assert_eq!(shader_id.get(), std::u32::MAX);
        let shader = builder.build();
        assert_ne!(shader_id.get(), std::u32::MAX);
        assert_eq!(shader_id.get(), shader.id());
    }
}
