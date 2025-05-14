use std::os::raw::c_void;

use super::gl::{self, STATIC_DRAW};

pub struct VertexBuffer {
    pub id: u32,
    pub vertex_count: i32,
}

impl VertexBuffer {
    pub fn new(vertices: &[f32]) -> Self {
        let mut id = 0;

        unsafe {
            gl::CreateBuffers(1, &mut id);
            gl::NamedBufferData(id, size_of_val(vertices) as isize, vertices.as_ptr() as *const c_void, STATIC_DRAW);
        }

        Self {
            id,
            vertex_count: (vertices.len() / 3) as i32,
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
