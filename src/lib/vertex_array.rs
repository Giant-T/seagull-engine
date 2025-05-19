use log::info;

use super::{
    gl::{self, FALSE, FLOAT},
    vertex_buffer::VertexBuffer,
};

pub struct VertexArray {
    pub id: u32,
    vertex_buffer: VertexBuffer,
}

impl VertexArray {
    pub fn new(vertex_buffer: VertexBuffer) -> Self {
        let mut id = 0;

        unsafe {
            gl::CreateVertexArrays(1, &mut id);
            gl::VertexArrayAttribFormat(id, 0, 3, FLOAT, FALSE, 0);
            gl::VertexArrayVertexBuffer(id, 0, vertex_buffer.id, 0, (size_of::<f32>() * 3) as i32);
            gl::VertexArrayAttribBinding(id, 0, 0);
            gl::EnableVertexArrayAttrib(id, 0);
        }

        info!("Initialized vertex array {id}");

        Self { id, vertex_buffer }
    }

    pub fn draw(&self, mode: u32) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawArrays(mode, 0, self.vertex_buffer.vertex_count);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
