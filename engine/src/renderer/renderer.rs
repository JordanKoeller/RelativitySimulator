use crate::utils::*;
use either::Either;
use std::clone::Clone;
use std::collections::HashMap;
use std::ffi::{CStr, CString};

use specs::prelude::*;

use crate::datastructures::KeyValueBuilder;
use crate::debug::*;
use crate::graphics::{
    AssetLibrary, AttributeType, BufferConfig, BufferLayout, DataBuffer, DataBufferBuilder, IndexBuffer,
    IndexBufferBuilder, MaterialComponent, MeshComponent, Shader, ShaderBuilder, Uniform, UniformLifecycle,
    VertexArray, VertexArrayBuilder,
};
use crate::platform::Window;
use crate::renderer::render_pipeline::*;
use crate::renderer::{Framebuffer, RenderQueueConsumer, RendererConfig};

use crate::ecs::Camera;

use crate::events::{Event, EventChannel, EventPayload, KeyCode, ReceiverID, StatelessEventChannel, WindowEvent};
use crate::physics::TransformComponent;

type TransformStack = Vec<Mat4F>;

struct Screen {
    pub screen_quad: VertexArray,
    pub shader: Shader,
    pub framebuffer: Framebuffer,
}

impl Screen {
    pub fn new(x_dim: i32, y_dim: i32) -> Self {
        let verts = vec![
            // Positions  // uv
            -1f32, 1f32, 0f32, 1f32, -1f32, -1f32, 0f32, 0f32, 1f32, -1f32, 1f32, 0f32, -1f32, 1f32, 0f32, 1f32, 1f32,
            -1f32, 1f32, 0f32, 1f32, 1f32, 1f32, 1f32,
        ];
        let inds = vec![0, 1, 2, 3, 4, 5];
        let screen_quad = VertexArrayBuilder::default()
            .with_vertex_buffer(
                DataBufferBuilder::default()
                    .with_layout(BufferLayout::new(vec![AttributeType::Float2, AttributeType::Float2]))
                    .with_data(verts)
                    .with_config(BufferConfig::static_vbo()),
            )
            .with_index_buffer(IndexBufferBuilder::default().with_data(inds))
            .build();
        let shader = ShaderBuilder::default()
            .with_source_file("shaders/screen_shader.glsl")
            .build();
        Self {
            framebuffer: Framebuffer::dims(x_dim, y_dim),
            shader,
            screen_quad,
        }
    }

    pub fn bind_draw(&self) {
        self.shader.bind();

        self.framebuffer.bind_texture_slot(1);
        let uniform = Uniform::Int(1);
        self.shader.set_uniform("tex", &uniform);
        self.screen_quad.bind();
        self.screen_quad.draw(self.shader.element_type());
        self.shader.unbind();
    }
}

pub struct Renderer {
    // Screen
    screen: Screen,
    // Shader/Uniform Management
    config_uniforms: HashMap<String, Uniform>, // Long-term uniforms
    common_uniforms: HashMap<String, Uniform>, // common uniforms, change every frame

    // Config
    config: RendererConfig,
    receiver_id: ReceiverID,
    // Transform Stack
}

impl Default for Renderer {
    fn default() -> Self {
        Renderer {
            screen: Screen::new(1600, 1200),
            config_uniforms: HashMap::new(),
            common_uniforms: HashMap::new(),
            config: RendererConfig::default(),
            receiver_id: 0,
        }
    }
}

impl Renderer {
    // Constructor
    pub fn new(screen_dims: Vec2F, receiver_id: ReceiverID) -> Renderer {
        Renderer {
            screen: Screen::new(screen_dims.x as i32, screen_dims.y as i32),
            config_uniforms: HashMap::new(),
            common_uniforms: HashMap::new(),
            config: RendererConfig::default(),
            receiver_id,
        }
    }

    // Some general getters/setters
    pub fn set_dims(&mut self, dims: Vec2F) {
        self.screen.framebuffer.resize(Vec2I::new(dims.x as i32, dims.y as i32));
    }

    pub fn submit_common_uniform(&mut self, name: &str, uniform: Uniform, lifecycle: UniformLifecycle) {
        match lifecycle {
            UniformLifecycle::Frame => {
                self.common_uniforms.insert(name.to_string(), uniform);
            }
            UniformLifecycle::Runtime => {
                self.config_uniforms.insert(name.to_string(), uniform);
            }
        }
    }

    pub fn submit_config(&mut self, config: RendererConfig) {
        self.submit_common_uniform(
            "lorentzFlag",
            Uniform::Int(config.relativity_mode()),
            UniformLifecycle::Runtime,
        );
        self.config = config;
    }

    // Methods that do something instead of just get/set things

    pub fn start_scene<'a>(&mut self, camera: &Camera, _timestep: &Timestep) {
        // self.process_all_events();
        self.extract_camera_uniforms(&camera);

        // #[cfg(feature = "debug")]
        // self.ui_renderer.add_diagnostics_pannel(camera, timestep, &self.config);
    }

    pub fn init_frame(&mut self, window: &mut Window) {
        self.screen.framebuffer.bind();
        unsafe {
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::Enable(gl::DEPTH_TEST);
        }
        window.clear_intrinsic_canvas();
    }

    pub fn end_frame(&mut self, window: &mut Window) {
        self.screen.framebuffer.unbind();

        window.clear_framebuffer();
        self.screen.bind_draw();
        window.swap_buffers();
    }

    pub fn render_scene<'a, 'b>(
        &mut self,
        mut queue: RenderQueueConsumer<'b>,
        materials: &ReadStorage<'a, MaterialComponent>,
        transforms: &ReadStorage<'a, TransformComponent>,
        assets: &mut Write<'a, AssetLibrary>,
    ) -> u32 {
        // unsafe {
        //     gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        // }
        let mut draw_call_count = 0u32;
        let pipeline_opt = RenderPipeline::<'_, ReadyToDrawStep>::new(&mut queue, assets);
        if let Some(pipeline) = pipeline_opt {
            let mut active_pipeline = pipeline.bind_global_uniforms(&[&self.config_uniforms, &self.common_uniforms]);
            loop {
                let saturated = active_pipeline.intake_queue(&mut queue, materials, transforms);
                let flushed = saturated.flush();
                draw_call_count += 1;
                if queue.consumed() {
                    self.common_uniforms.clear();
                    break;
                } else {
                    let proceeded = flushed.proceed(&mut queue);
                    active_pipeline = match proceeded {
                        Either::Left(ready_q) => {
                            ready_q.bind_global_uniforms(&[&self.config_uniforms, &self.common_uniforms])
                        }
                        Either::Right(next_q) => next_q,
                    };
                }
            }
        }
        // unsafe {
        //     gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        // }
        draw_call_count
    }

    // Private helper functions

    fn extract_camera_uniforms(&mut self, camera: &Camera) {
        self.common_uniforms
            .insert("view".to_string(), Uniform::Mat4(*camera.view_matrix()));
        let f32_dims = Vec2F::new(
            self.screen.framebuffer.spec.dims.x as f32,
            self.screen.framebuffer.spec.dims.y as f32,
        );
        self.common_uniforms.insert(
            "projection".to_string(),
            Uniform::Mat4(camera.projection_matrix(&f32_dims)),
        );
        self.common_uniforms
            .insert("beta".to_string(), Uniform::Float(camera.beta()));
        self.common_uniforms
            .insert("gamma".to_string(), Uniform::Float(camera.gamma()));
        self.common_uniforms
            .insert("light_ambient".to_string(), Uniform::Vec3(Vec3F::new(1.0, 1.0, 1.0)));
        self.common_uniforms
            .insert("light_diffuse".to_string(), Uniform::Vec3(Vec3F::new(1.0, 1.0, 1.0)));
        self.common_uniforms
            .insert("light_specular".to_string(), Uniform::Vec3(Vec3F::new(1.0, 1.0, 1.0)));
        self.common_uniforms.insert(
            "camera_position".to_string(),
            Uniform::Vec3(Vec3F::new(200.0, 200.0, 200.0)),
        );
        self.common_uniforms.insert(
            "light_position".to_string(),
            Uniform::Vec3(Vec3F::new(200.0, 200.0, -200.0)),
        );
        // self.common_uniforms.insert(
        //   "camera_position".to_string(),
        //   Uniform::Vec3(camera.position.clone()),
        // );
        // self.common_uniforms.insert(
        //     "changeOfBasis".to_string(),
        //     Uniform::Mat3(camera.velocity_basis_matrix()),
        // );
        // self.common_uniforms.insert(
        //     "changeOfBasisInverse".to_string(),
        //     Uniform::Mat3(camera.velocity_inverse_basis_matrix()),
        // );
    }

    pub fn process_events(&mut self, chanel: &mut StatelessEventChannel<WindowEvent>) {
        let id = self.receiver_id;
        chanel.for_each(&id, |window_event| match &window_event.code {
            Event::WindowResized => {
                if let Some(payload) = &window_event.payload {
                    match payload {
                        EventPayload::WindowSize(new_sz) => self.set_dims(new_sz.clone()),
                        _ => {}
                    }
                }
            }
            Event::KeyPressed(KeyCode::Tab) => {
                let mut new_config = self.config.clone();
                new_config.mode = new_config.mode.rotate();
                self.submit_config(new_config);
            }
            Event::KeyPressed(KeyCode::Q) => {
                let mut new_config = self.config.clone();
                new_config.debug = !new_config.debug;
                self.submit_config(new_config);
            }
            _ => {}
        });
    }
}
