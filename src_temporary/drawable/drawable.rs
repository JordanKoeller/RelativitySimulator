use shader_manager::ShaderManager;
use std::cmp::Ord;
use std::cmp::Ordering;

pub trait Drawable {
  fn draw(&self, shader: &ShaderManager);
  fn pre_draw(&self, shader: &ShaderManager) {
    let s = shader.get_shader(self.shader_name());
    s.use_program();
  }
  fn shader_name(&self) -> String;


}

impl PartialOrd for dyn Drawable {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for dyn Drawable {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.shader_name() == other.shader_name()
  }
}

impl Eq for dyn Drawable {}

impl Ord for dyn Drawable {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.shader_name().cmp(&other.shader_name())
  }
}
