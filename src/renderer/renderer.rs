use either::Either;
use std::clone::Clone;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use utils::*;

use specs::prelude::*;

use debug::*;
use renderer::platform::VertexArray;
use renderer::render_pipeline::*;
use renderer::*;

use ecs::{Camera, DrawableId, Material, MeshComponent};

use events::{Event, EventChannel, EventPayload, KeyCode, ReceiverID, StatelessEventChannel, WindowEvent};
use physics::TransformComponent;

type TransformStack = Vec<Mat4F>;

struct Screen {
    pub screen_quad: VertexArray,
    pub shader: Shader,
    pub framebuffer: Framebuffer,
}

pub struct Renderer {
    // Screen
    screen: Screen,
    // Shader/Uniform Management
    config_uniforms: HashMap<CString, Uniform>, // Long-term uniforms
    common_uniforms: HashMap<CString, Uniform>, // common uniforms, change every frame

    // Asset management
    pub assets: AssetLibrary,

    // Config
    config: RendererConfig,
    receiver_id: ReceiverID,
    // Transform Stack
}

impl Default for Renderer {
    fn default() -> Self {
        Renderer {
            screen: create_screen(1600, 1200),
            config_uniforms: HashMap::new(),
            common_uniforms: HashMap::new(),
            assets: AssetLibrary::default(),
            config: RendererConfig::default(),
            receiver_id: 0,
        }
    }
}

impl Renderer {
    // Constructor
    pub fn new(screen_dims: Vec2F, channel: &mut StatelessEventChannel<WindowEvent>) -> Renderer {
        let receiver_id = channel.register_with_subs(&[
            WindowEvent::new(Event::WindowResized),
            WindowEvent::new(Event::KeyPressed(KeyCode::Tab)),
            WindowEvent::new(Event::KeyPressed(KeyCode::Q)),
        ]);
        Renderer {
            screen: create_screen(screen_dims.x as i32, screen_dims.y as i32),
            config_uniforms: HashMap::new(),
            common_uniforms: HashMap::new(),
            assets: AssetLibrary::default(),
            config: RendererConfig::default(),
            receiver_id,
        }
    }

    // Some general getters/setters
    pub fn set_dims(&mut self, dims: Vec2F) {
        self.screen.framebuffer.resize(Vec2I::new(dims.x as i32, dims.y as i32));
    }

    pub fn submit_common_uniform(&mut self, name: CString, uniform: Uniform, lifecycle: UniformLifecycle) {
        match lifecycle {
            UniformLifecycle::Frame => {
                self.common_uniforms.insert(name, uniform);
            }
            UniformLifecycle::Runtime => {
                self.config_uniforms.insert(name, uniform);
            }
        }
    }

    pub fn submit_shader(&mut self, shader: Shader) {
        self.assets.register_shader(shader);
    }

    pub fn submit_model(&mut self, mut model: Mesh) -> DrawableId {
        model.refresh();
        self.assets.register_asset(model)
    }

    pub fn submit_config(&mut self, config: RendererConfig) {
        self.submit_common_uniform(
            CString::from(c_str!("lorentzFlag")),
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
        window.clear_framebuffer();
    }

    pub fn end_frame(&mut self, window: &mut Window) {
        self.screen.framebuffer.unbind();
        unsafe {
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::Disable(gl::DEPTH_TEST);
        }
        window.clear_framebuffer2();
        self.screen.shader.bind();
        self.screen
            .shader
            .set_texture(1, c_str!("tex"), &self.screen.framebuffer.texture());
        self.screen.screen_quad.bind();
        self.screen.screen_quad.draw(&self.screen.shader.element_type);
        self.screen.shader.unbind();

        window.swap_buffers();
    }

    pub fn render_scene<'a>(
        &mut self,
        mut queue: RenderQueueConsumer<'a>,
        materials: &ReadStorage<'a, Material>,
        transforms: &ReadStorage<'a, TransformComponent>,
    ) -> u32 {
        let mut draw_call_count = 0u32;
        let pipeline_opt = RenderPipeline::<'_, ReadyToDrawStep>::new(&mut queue, &mut self.assets);
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
        draw_call_count
    }

    // Private helper functions

    fn extract_camera_uniforms(&mut self, camera: &Camera) {
        self.common_uniforms
            .insert(CString::new("view").unwrap(), Uniform::Mat4(*camera.view_matrix()));
        let f32_dims = Vec2F::new(
            self.screen.framebuffer.spec.dims.x as f32,
            self.screen.framebuffer.spec.dims.y as f32,
        );
        self.common_uniforms.insert(
            CString::new("projection").unwrap(),
            Uniform::Mat4(camera.projection_matrix(&f32_dims)),
        );
        self.common_uniforms
            .insert(CString::new("beta").unwrap(), Uniform::Float(camera.beta()));
        self.common_uniforms
            .insert(CString::new("gamma").unwrap(), Uniform::Float(camera.gamma()));
        // self.common_uniforms.insert(
        //   CString::new("cameraPos").unwrap(),
        //   Uniform::Vec3(camera.position.clone()),
        // );
        self.common_uniforms.insert(
            CString::new("changeOfBasis").unwrap(),
            Uniform::Mat3(camera.velocity_basis_matrix()),
        );
        self.common_uniforms.insert(
            CString::new("changeOfBasisInverse").unwrap(),
            Uniform::Mat3(camera.velocity_inverse_basis_matrix()),
        );
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

fn create_screen(w: i32, h: i32) -> Screen {
    let verts = [
        // Positions  // uv
        -1f32, 1f32, 0f32, 1f32, -1f32, -1f32, 0f32, 0f32, 1f32, -1f32, 1f32, 0f32, -1f32, 1f32, 0f32, 1f32, 1f32,
        -1f32, 1f32, 0f32, 1f32, 1f32, 1f32, 1f32,
    ];

    let inds = vec![0, 1, 2, 3, 4, 5];
    let mut screen_quad = VertexArray::new(
        DataBuffer::static_buffer(
            &verts,
            BufferLayout::new(vec![AttributeType::Float2, AttributeType::Float2]),
        ),
        IndexBuffer::create(inds),
    );
    let shader = Shader::from_file("renderer_screen", "shaders/screen_shader.glsl");
    screen_quad.refresh();

    Screen {
        framebuffer: Framebuffer::dims(w, h),
        shader,
        screen_quad,
    }
}
