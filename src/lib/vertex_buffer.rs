use std::sync::Arc;

use anyhow::Result;
use bytemuck::cast_slice;
use glow::{HasContext, STATIC_DRAW};
use log::info;

pub struct VertexBuffer {
    pub id: glow::Buffer,
    pub vertex_count: i32,
    gl: Arc<glow::Context>,
}

impl VertexBuffer {
    pub fn new(gl: Arc<glow::Context>, vertices: &[f32]) -> Result<Self> {
        let id;

        unsafe {
            id = gl.create_named_buffer().or_else(|s| Err(anyhow::anyhow!(s)))?;
            gl.named_buffer_data_u8_slice(id, cast_slice(vertices), STATIC_DRAW);
        }

        info!("Initialized vertex buffer {id:?}");

        Ok(Self {
            id,
            vertex_count: (vertices.len() / 3) as i32,
            gl,
        })
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.id);
        }
    }
}
