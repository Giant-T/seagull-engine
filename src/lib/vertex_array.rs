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

        Self { id, vertex_buffer }
    }
}
