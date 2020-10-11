
use utils::*;

use renderer::modeling::DrawableMemo;

#[derive(Clone, Debug)]
pub enum RenderCommand {
  SingleDrawable(DrawableMemo),
  // MultiDrawable(Ref<Vec<Ref<dyn Drawable>>>),
}

// pub struct RenderCommand {
//   pub drawable: Ref<dyn Drawable>,
// }

impl RenderCommand {
  pub fn from(drawable: DrawableMemo) -> RenderCommand {
    RenderCommand::SingleDrawable(drawable)
  }
  // pub fn from_multi(drawables: Ref<Vec<Ref<dyn Drawable>>>) -> RenderCommand {
  //   RenderCommand::MultiDrawable(drawables)
  // }
}