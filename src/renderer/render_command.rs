
use utils::*;

use renderer::modeling::Drawable;

pub struct RenderCommand {
  pub drawable: Ref<dyn Drawable>,
}

impl RenderCommand {
  pub fn from(drawable: Ref<dyn Drawable>) -> RenderCommand {
    RenderCommand {
      drawable
    }
  }
}