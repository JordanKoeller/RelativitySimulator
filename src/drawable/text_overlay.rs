use drawable::Drawable;
use shader_manager::ShaderManager;
use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::c_void;
use freetype::Library;

// extern crate freetype;
use cgmath::Vector2;

#[derive(Copy, Clone)]
struct Character {
  texture_id: u32,
  size: Vector2<i32>,
  bearing: Vector2<i32>,
  advance: i64,
}

struct StaticVars {
  alphabet: std::collections::HashMap<usize, Character>,
  vao: u32,
  vbo: u32
}

lazy_static! {
  static ref CHARACTER_RENDERER: StaticVars  = init_alphabet();
}

fn init_alphabet() -> StaticVars {
  // The three return values.
  let mut vao = 0;
  let mut vbo = 0;

  let mut alphabet = std::collections::HashMap::new();

  // Following initializes everything
  let lib = Library::init().expect("Failed to init library");
  let face = lib
    .new_face("/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf", 0)
    .expect("Failed to init FreeType face");
  face.set_pixel_sizes(0, 48).unwrap();
  let exclusions =[86, 87, 89, 92, 118, 119, 32];
  for c in 32..127 {
    let has_char = face.load_char(c, freetype::face::LoadFlag::RENDER).err();
    match has_char {
      None => {}
      Some(e) => {
        println!("Encountered exception {}", e);
        continue;
      }
    }
    if !exclusions.contains(&c) {
      let mut texture_id = 0;
      unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexImage2D(
          gl::TEXTURE_2D,
          0,
          gl::RED as i32,
          face.glyph().bitmap().width(),
          face.glyph().bitmap().rows(),
          0,
          gl::RED,
          gl::UNSIGNED_BYTE,
          &face.glyph().bitmap().buffer()[0] as *const u8 as *const c_void,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
      }
      let character = Character {
        texture_id: texture_id,
        size: Vector2 {
          x: face.glyph().bitmap().width(),
          y: face.glyph().bitmap().rows(),
        },
        bearing: Vector2 {
          x: face.glyph().bitmap_left(),
          y: face.glyph().bitmap_top(),
        },
        advance: face.glyph().advance().x,
      };
      alphabet.insert(c, character);
    }
  }
  unsafe {
    gl::BindTexture(gl::TEXTURE_2D, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::BindVertexArray(vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
      gl::ARRAY_BUFFER,
      (24 * size_of::<f32>()) as isize,
      0 as *const c_void,
      gl::DYNAMIC_DRAW,
    );
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
      0,
      4,
      gl::FLOAT,
      gl::FALSE,
      (4 * size_of::<f32>()) as i32,
      0 as *const c_void,
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);
  }

  // let projection = cgmath::ortho(0.0, 800.0, 0.0, 600.0);

  StaticVars {
    alphabet: alphabet, vao: vao, vbo: vbo}
}

// fn init_vao_vbo() -> cgmath::Vector2<u32> {

//   unsafe {
//     gl::GenVertexArrays(1, &mut vao);
//     gl::GenBuffers(1, &mut vbo);
//     gl::BindVertexArray(vao);
//     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//     gl::BufferData(
//       gl::ARRAY_BUFFER,
//       (24 * size_of::<f32>()) as isize,
//       0 as *const c_void,
//       gl::DYNAMIC_DRAW,
//     );
//     gl::EnableVertexAttribArray(0);
//     gl::VertexAttribPointer(
//       0,
//       4,
//       gl::FLOAT,
//       gl::FALSE,
//       (4 * size_of::<f32>()) as i32,
//       0 as *const c_void,
//     );
//     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
//     gl::BindVertexArray(0);
//   }
//   cgmath::Vector2::<u32> { x: vao, y: vbo }
// }

pub struct TextOverlay {
  data: String,
  x: f32,
  y: f32,
  scale: f32,
}

impl TextOverlay {
  pub fn new() -> TextOverlay {
    TextOverlay {
      data: "Some data".to_string(),
      x: 0.0,
      y: 0.0,
      scale: 1.0,
    }
  }

  pub fn set_data(&mut self, txt: String, x: f32, y: f32) {
    self.data = txt.replace(" ", "_");
    self.x = x;
    self.y = y;
  }
}

impl Drawable for TextOverlay {
  fn shader_name(&self) -> String {
    "text".to_string()
  }
  
  fn draw(&self, shader: &ShaderManager) {
    let shader = shader.get_shader(self.shader_name());
    let mut pos_x = self.x;
    shader.use_program();
    let projection: cgmath::Matrix4<f32> = cgmath::ortho(0.0, 1600.0, 0.0, 1200.0, 0.0, 1.0);
    shader.set_mat4(c_str!("projection"), &projection);
    shader.set_vec3(c_str!("textColor"), 0.0, 0.0, 1.0);
    unsafe {
      gl::Enable(gl::BLEND);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindVertexArray(CHARACTER_RENDERER.vao);
    }
    for c in self.data.chars() {
      let c_i = c as usize;
      let ch_opt = CHARACTER_RENDERER.alphabet.get(&c_i);
      match ch_opt {
        None => continue,
        Some(ch) => {
          let xpos = pos_x + ch.bearing.x as f32 * self.scale;
          let ypos = self.y - (ch.size.y - ch.bearing.y) as f32 * self.scale;
          let w = ch.size.x as f32 * self.scale;
          let h = ch.size.y as f32 * self.scale;
          let vertices = [
            xpos,
            ypos + h,
            0.0,
            0.0,
            xpos,
            ypos,
            0.0,
            1.0,
            xpos + w,
            ypos,
            1.0,
            1.0,
            xpos,
            ypos + h,
            0.0,
            0.0,
            xpos + w,
            ypos,
            1.0,
            1.0,
            xpos + w,
            ypos + h,
            1.0,
            0.0,
          ];
          unsafe {
            gl::BindTexture(gl::TEXTURE_2D, ch.texture_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, CHARACTER_RENDERER.vbo);
            gl::BufferSubData(
              gl::ARRAY_BUFFER,
              0,
              (vertices.len() * size_of::<f32>()) as isize,
              &vertices[0] as *const f32 as *const c_void,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            pos_x += (ch.advance >> 6) as f32 * self.scale;
          }
        }
      }
    }
    unsafe {
      gl::BindVertexArray(0);
      gl::BindTexture(gl::TEXTURE_2D, 0);
      gl::Disable(gl::BLEND);
    }
  }
}
