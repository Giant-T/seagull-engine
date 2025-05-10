use std::os::raw::c_void;

use super::gl::{self, STATIC_DRAW};

pub struct VertexBuffer {
    pub id: u32,
    vertices: Box<[f32]>,
}

impl VertexBuffer {
    pub fn new(vertices: Box<[f32]>) -> Self {
        let mut id = 0;

        unsafe {
            gl::CreateBuffers(1, &mut id);
            gl::NamedBufferData(id, vertices.len() as isize, vertices.as_ptr() as *const c_void, STATIC_DRAW);
        }

        Self {
            id,
            vertices
        }
    }
}
