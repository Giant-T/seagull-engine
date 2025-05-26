use std::sync::Arc;

use anyhow::Result;
use glm::Vec2;
use glow::{HasContext, UniformLocation, FRAMEBUFFER, TRIANGLE_FAN};
use seagull_lib::{shader::Shader, vertex_array::VertexArray, vertex_buffer::VertexBuffer};

pub struct Voronoi {
    points: Vec<Vec2>,
    shader_program: Shader,
    points_loc: UniformLocation,
    vertex_array: VertexArray,
    gl: Arc<glow::Context>,
}

impl Voronoi {
    pub fn new(gl: Arc<glow::Context>, points: Vec<Vec2>) -> Result<Self> {
        let shader_program = Shader::new(
            gl.clone(),
            include_str!("../VS.glsl"),
            include_str!("FS.glsl"),
        )?;

        let points_loc = shader_program.get_loc("Points")?;
        shader_program.uniform_2f_slice(&points_loc, &points);

        let vertices = [
            -1.0, -1.0, 0.0, // bottom left
            -1.0, 1.0, 0.0, // top left
            1.0, 1.0, 0.0, // top right
            1.0, -1.0, 0.0, // bottom right
        ];

        let vertex_buffer = VertexBuffer::new(gl.clone(), &vertices)?;
        let vertex_array = VertexArray::new(gl.clone(), vertex_buffer)?;

        Ok(Self {
            points,
            shader_program,
            points_loc,
            vertex_array,
            gl,
        })
    }

    pub fn apply(&self, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        unsafe {
            self.gl.viewport(x, y, width, height);
            self.gl.bind_framebuffer(FRAMEBUFFER, None);
        }

        self.shader_program.use_program();
        self.vertex_array.draw(TRIANGLE_FAN);

        Ok(())
    }
}
