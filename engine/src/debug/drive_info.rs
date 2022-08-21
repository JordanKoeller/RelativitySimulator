use std::ptr;

use glfw::Context;
use imgui::Context as ImContext;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;

pub fn print_limits() {
  println!("======================================================================");
  println!("                             OpenGL Limits                            ");
  println!("======================================================================");
  unsafe {
    let mut value = 0;
    gl::GetIntegerv(gl::MAX_VERTEX_UNIFORM_COMPONENTS, &mut value);
    println!("Vertex Uniform Limit:       {}", value);
    gl::GetIntegerv(gl::MAX_GEOMETRY_UNIFORM_COMPONENTS, &mut value);
    println!("Geometry Uniform Limit:     {}", value);
    gl::GetIntegerv(gl::MAX_FRAGMENT_UNIFORM_COMPONENTS, &mut value);
    println!("Fragment Uniform Limit:     {}", value);
    gl::GetIntegerv(gl::MAX_TESS_CONTROL_UNIFORM_COMPONENTS, &mut value);
    println!("Tess Control Uniform Limit: {}", value);
    gl::GetIntegerv(gl::MAX_TESS_EVALUATION_UNIFORM_COMPONENTS, &mut value);
    println!("Tess Eval Uniform Limit:    {}", value);
    gl::GetIntegerv(gl::MAX_COMPUTE_UNIFORM_COMPONENTS, &mut value);
    println!("Compute Uniform Limit:      {}", value);
    gl::GetIntegerv(gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS, &mut value);
    println!("Max Global texture slots:   {}", value);
    gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut value);
    println!("Max textures per stage:     {}", value);
  }
  println!("======================================================================");
}
