use std::{sync::Arc, time::Instant};

use anyhow::Result;
use glow::{TEXTURE0, TRIANGLE_FAN, UniformLocation};
use log::info;
use winit::dpi::PhysicalSize;

use seagull_lib::{
    frame_buffer::FrameBuffer, shader::Shader, vertex_array::VertexArray,
    vertex_buffer::VertexBuffer,
};

pub struct Pixelate {
    scale: f32,
    fbo: FrameBuffer,
    fragment_shader: Shader,
    display_shader: Shader,
    texture_loc: UniformLocation,
    elapsed_loc: UniformLocation,
    vertex_array: VertexArray,
    start: Instant,
}

impl Pixelate {
    pub fn new(gl: Arc<glow::Context>, size: &PhysicalSize<u32>, scale: f32) -> Result<Self> {
        let fbo = FrameBuffer::new(
            gl.clone(),
            (size.width as f32 / scale) as i32,
            (size.height as f32 / scale) as i32,
        )?;
        let fragment_shader =
            Shader::new(gl.clone(), include_str!("VS.glsl"), include_str!("FS.glsl"))?;
        let elapsed_loc = fragment_shader.get_loc("Elapsed")?;

        let display_shader = Shader::new(
            gl.clone(),
            include_str!("VS.glsl"),
            include_str!("Display-FS.glsl"),
        )?;
        let texture_loc = display_shader.get_loc("FBO")?;

        let vertices = [
            -1.0, -1.0, 0.0, // bottom left
            -1.0, 1.0, 0.0, // top left
            1.0, 1.0, 0.0, // top right
            1.0, -1.0, 0.0, // bottom right
        ];
        let vertex_buffer = VertexBuffer::new(gl.clone(), &vertices)?;
        let vertex_array = VertexArray::new(gl.clone(), vertex_buffer)?;

        info!("Initialized pixelate effect");

        Ok(Self {
            scale,
            fbo,
            fragment_shader,
            display_shader,
            elapsed_loc,
            texture_loc,
            vertex_array,
            start: Instant::now(),
        })
    }

    pub fn apply(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        _source: Option<&FrameBuffer>,
        target: Option<&FrameBuffer>,
    ) -> Result<()> {
        let time_since_start = self.start.elapsed();
        self.fbo.bind();
        self.fragment_shader.use_program();
        self.fragment_shader
            .uniform_1f(&self.elapsed_loc, time_since_start.as_millis() as f32);
        self.vertex_array.draw(TRIANGLE_FAN);
        self.fbo.unbind(x, y, width, height);

        if let Some(target) = target {
            target.bind();
        }

        self.display_shader.use_program();
        self.fbo.texture.activate_texture(TEXTURE0);
        self.display_shader.uniform_1i(&self.texture_loc, 0);
        self.vertex_array.draw(TRIANGLE_FAN);

        if let Some(target) = target {
            target.unbind(x, y, width, height);
        }

        Ok(())
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) -> Result<()> {
        self.fbo.resize(
            (size.width as f32 / self.scale) as i32,
            (size.height as f32 / self.scale) as i32,
        )
    }
}
