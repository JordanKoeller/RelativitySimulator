use std::mem::size_of;
use std::os::raw::c_void;

use gl;

use initializers::mesh_spec::{AttributeTypes, ElementSpec, GLLayout, GLSpec};
use initializers::Factory;
use renderer::GLBuffer;

pub struct GLBufferFactory {}

impl Default for GLBufferFactory {
    fn default() -> GLBufferFactory {
        GLBufferFactory {}
    }
}

impl GLBufferFactory {

    pub fn get_buffer(&self, input_buff: Vec<f32>, inds: Vec<u32>, attributes: Vec<AttributeTypes>) -> GLBuffer {
        let element_spec = Box::new(ElementSpec::new(attributes));
        let buff = input_buff; // Will change in the future when I start calculating normals
        self.allocate_buffer(&buff, &inds, element_spec)
    }

    // TODO: Add method for constructing a mesh from mesh spec.
    pub fn new_gl_buffer(&self, spec: GLSpec) -> GLBuffer {
        self.allocate_buffer(&spec.points_buffer, &spec.inds_buffer, Box::new(spec.elem_spec))
    }

    fn allocate_buffer(&self, buf: &Vec<f32>, inds: &Vec<u32>, layout: Box<dyn GLLayout>) -> GLBuffer {
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        let buff_sz = (buf.len() * size_of::<f32>()) as isize;
        let buff_data = &buf[0] as *const f32 as *const c_void;
        let inds_sz = (inds.len() * size_of::<u32>()) as isize;
        let inds_data = &inds[0] as *const u32 as *const c_void;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            gl::BindVertexArray(vao);
            // Bind data buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, buff_sz, buff_data, gl::STATIC_DRAW);
            // Bind element buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, inds_sz, inds_data, gl::STATIC_DRAW);

            // Specify all the attributes.
            let attrib_sizes = layout.attrib_lengths();
            let attrib_offsets = layout.attrib_offsets();
            let elem_sz = layout.elem_length() * size_of::<f32>() as i32;
            for i in 0..attrib_offsets.len() {
                println!("Setting Attrib {} sz {} offset {}", i, attrib_sizes[i], attrib_offsets[i]);
                gl::VertexAttribPointer(
                    i as u32,
                    attrib_sizes[i],
                    gl::FLOAT,
                    gl::FALSE,
                    elem_sz,
                    attrib_offsets[i] as *const i32 as *const c_void,
                );
                gl::EnableVertexAttribArray(i as u32);
            }
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // Umbind array, just to clean up
            gl::BindVertexArray(0);
        }
        GLBuffer {
            vao: vao,
            vbo: vbo,
            ebo: ebo,
            num_elems: inds.len() as i32,
        }
    }
}

impl Factory for GLBufferFactory {
    type Resource = GLBuffer;
    type Spec = GLSpec;
    fn new_resource(&self, spec: Self::Spec) -> Self::Resource {
        self.new_gl_buffer(spec)
    }
}
