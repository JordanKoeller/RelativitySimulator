use crate::utils::*;
use either::Either;
use std::clone::Clone;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::RwLockReadGuard;

use specs::prelude::*;

use crate::datastructures::{AVLTree, AVLTreeIterator, RegistryItem};
use crate::debug::*;
use crate::graphics::{
    AssetLibrary, AttributeType, BufferConfig, BufferLayout, DataBuffer, DataBufferBuilder, IndexBuffer,
    IndexBufferBuilder, MaterialComponent, MeshComponent, Shader, ShaderBuilder, Uniform, UniformLifecycle,
    VertexArray, VertexArrayBuilder,
};
use crate::platform::{Screen, Window};
use crate::renderer::render_pipeline::*;
use crate::renderer::{DrawCall, Framebuffer, RenderQueueConsumer, RendererConfig, PolygonMode};

use crate::ecs::Camera;

use crate::events::{Event, EventChannel, EventPayload, KeyCode, ReceiverID, StatelessEventChannel, WindowEvent};
use crate::physics::TransformComponent;

type TransformStack = Vec<Mat4F>;

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
            screen: Screen::new(1920, 1080),
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

    pub fn submit_env_uniform(&mut self, name: &str, uniform: Uniform) {
        self.common_uniforms.insert(name.to_string(), uniform);
    }

    pub fn submit_config(&mut self, config: RendererConfig) {
        self.config = config;
    }

    // Methods that do something instead of just get/set things

    pub fn start_scene<'a>(&mut self, camera: &Camera) {
        self.extract_camera_uniforms(&camera);
    }

    pub fn init_frame(&mut self, window: &mut Window) {
        self.screen.bind_framebuffer();
        if self.config.polygon_mode == PolygonMode::LINE {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
        }
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
        window.clear_framebuffer();
    }

    pub fn end_frame(&mut self, window: &mut Window) {
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
        self.screen.unbind_framebuffer();
        window.clear_intrinsic_canvas();
        self.screen.draw_framebuffer_contents();
        window.swap_buffers();
    }

    pub fn render_scene<'a, 'b>(
        &mut self,
        render_queue: RwLockReadGuard<'b, AVLTree<DrawCall>>,
        materials: &ReadStorage<'a, MaterialComponent>,
        transforms: &ReadStorage<'a, TransformComponent>,
        assets: &mut Write<'a, AssetLibrary>,
        debug_metrics: &DebugMetrics,
    ) {
        debug_metrics.draw_calls.reset();
        debug_metrics.poly_count.reset();

        let mut queue = render_queue.iter();
        let pipeline_opt = RenderPipeline::<'_, ReadyToDrawStep>::new(&mut queue, assets);
        if let Some(pipeline) = pipeline_opt {
            let mut active_pipeline = pipeline.bind_global_uniforms(&[&self.config_uniforms, &self.common_uniforms]);
            let poly_count = loop {
                let saturated = active_pipeline.intake_queue(&mut queue, materials, transforms);
                let flushed = saturated.flush();
                debug_metrics.draw_calls.increment();
                if queue.empty() {
                    self.common_uniforms.clear();
                    break flushed.state.poly_count as u32;
                } else {
                    let proceeded = flushed.proceed(&mut queue);
                    active_pipeline = match proceeded {
                        Either::Left(ready_q) => {
                            ready_q.bind_global_uniforms(&[&self.config_uniforms, &self.common_uniforms])
                        }
                        Either::Right(next_q) => next_q,
                    };
                }
            };
            debug_metrics.poly_count.increment_by(poly_count);
        }
        // unsafe {
        //     gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        // }
    }

    // Private helper functions

    fn extract_camera_uniforms(&mut self, camera: &Camera) {
        self.common_uniforms
            .insert("view".to_string(), Uniform::Mat4(camera.view_matrix()));
        self.common_uniforms.insert(
            "projection".to_string(),
            Uniform::Mat4(camera.projection_matrix(self.screen.aspect_ratio())),
        );
        self.common_uniforms
            .insert("light_ambient".to_string(), Uniform::Vec3(Vec3F::new(1.0, 1.0, 1.0)));
        self.common_uniforms
            .insert("light_diffuse".to_string(), Uniform::Vec3(Vec3F::new(1.0, 1.0, 1.0)));
        self.common_uniforms
            .insert("light_specular".to_string(), Uniform::Vec3(Vec3F::new(1.0, 1.0, 1.0)));
        self.common_uniforms
            .insert("camera_position".to_string(), Uniform::Vec3(camera.position()));
        self.common_uniforms
            .insert("debug_line_length".to_string(), Uniform::Float(0.1));
        self.common_uniforms.insert(
            "light_position".to_string(),
            Uniform::Vec3(Vec3F::new(200.0, 200.0, -200.0)),
        );
    }

    pub fn process_events(&mut self, chanel: &mut StatelessEventChannel<WindowEvent>) {
        let id = self.receiver_id;
        chanel.for_each(&id, |window_event| match &window_event.code {
            Event::WindowResized => {
                if let Some(payload) = &window_event.payload {
                    match payload {
                        EventPayload::WindowSize(new_sz) => {
                            let vec_sz = Vec2I::new(new_sz.x as i32, new_sz.y as i32);
                            self.screen.set_framebuffer(Framebuffer::from_dims(vec_sz.x, vec_sz.y));
                        }
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
            Event::KeyPressed(KeyCode::One) => {
                let mut config = self.config.clone();
                config.polygon_mode = config.polygon_mode.rotate();
                println!("Setting polygon mode {:?}", config.polygon_mode);
                self.submit_config(config);
            }
            _ => {}
        });
    }
}
