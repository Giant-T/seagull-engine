use std::rc::Rc;

use anyhow::Result;
use glow::{FLOAT, HasContext};
use log::info;

use super::vertex_buffer::VertexBuffer;

pub struct VertexArray {
    pub id: glow::VertexArray,
    vertex_buffer: VertexBuffer,
    gl: Rc<glow::Context>,
}

impl VertexArray {
    pub fn new(gl: Rc<glow::Context>, vertex_buffer: VertexBuffer) -> Result<Self> {
        let id;

        unsafe {
            id = gl
                .create_named_vertex_array()
                .or_else(|s| Err(anyhow::anyhow!(s)))?;
            gl.vertex_array_attrib_format_f32(id, 0, 3, FLOAT, false, 0);
            gl.vertex_array_vertex_buffer(
                id,
                0,
                Some(vertex_buffer.id),
                0,
                (size_of::<f32>() * 3) as i32,
            );
            gl.vertex_array_attrib_binding_f32(id, 0, 0);
            gl.enable_vertex_array_attrib(id, 0);
        }

        info!("Initialized vertex array {id:?}");

        Ok(Self {
            id,
            vertex_buffer,
            gl,
        })
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.id));
        }
    }

    pub fn draw(&self, mode: u32) {
        unsafe {
            self.bind();
            self.gl
                .draw_arrays(mode, 0, self.vertex_buffer.vertex_count);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.id);
        }
    }
}
