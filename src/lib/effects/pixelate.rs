use anyhow::Result;
use std::{rc::Rc, time::Instant};
use winit::dpi::PhysicalSize;

use crate::lib::{
    app::App,
    extensions::Extensions,
    frame_buffer::FrameBuffer,
    gl::{self, FRAGMENT_SHADER, FRAGMENT_SHADER_BIT, TRIANGLE_FAN, VERTEX_SHADER},
    program::{Pipeline, Shader},
    vertex_array::VertexArray,
    vertex_buffer::VertexBuffer,
};

use super::Effect;

pub struct Pixelate {
    scale: f32,
    fbo: FrameBuffer,
    vertex_shader: Shader,
    fragment_shader: Shader,
    display_shader: Shader,
    texture_loc: i32,
    elapsed_loc: i32,
    pipeline: Pipeline,
    vertex_array: VertexArray,
    start: Instant,
}

impl Pixelate {
    pub fn new(extensions: Rc<Extensions>, size: &PhysicalSize<u32>, scale: f32) -> Result<Self> {
        let fbo = FrameBuffer::new(
            extensions,
            (size.width as f32 / scale) as i32,
            (size.height as f32 / scale) as i32,
        )?;
        let vertex_shader = Shader::new(VERTEX_SHADER, include_str!("VS.glsl"))?;
        let fragment_shader = Shader::new(FRAGMENT_SHADER, include_str!("FS.glsl"))?;
        let elapsed_loc = fragment_shader.get_loc("Elapsed")?;

        let display_shader = Shader::new(FRAGMENT_SHADER, include_str!("Display-FS.glsl"))?;
        let texture_loc = display_shader.get_loc("FBO")?;

        let pipeline = Pipeline::new(&vertex_shader, &fragment_shader)?;

        let vertices = [
            -1.0, -1.0, 0.0, // bottom left
            -1.0, 1.0, 0.0, // top left
            1.0, 1.0, 0.0, // top right
            1.0, -1.0, 0.0, // bottom right
        ];
        let vertex_buffer = VertexBuffer::new(&vertices);
        let vertex_array = VertexArray::new(vertex_buffer);

        Ok(Self {
            scale,
            fbo,
            vertex_shader,
            fragment_shader,
            display_shader,
            elapsed_loc,
            texture_loc,
            pipeline,
            vertex_array,
            start: Instant::now(),
        })
    }
}

impl Effect for Pixelate {
    fn apply(
        &self,
        app: &App,
        _source: Option<&FrameBuffer>,
        target: Option<&FrameBuffer>,
    ) -> Result<()> {
        let time_since_start = self.start.elapsed();
        self.fbo.bind();
        // TODO: add methods to shader to set uniforms
        unsafe {
            gl::ProgramUniform1f(
                self.fragment_shader.id,
                self.elapsed_loc,
                time_since_start.as_millis() as f32,
            );
        }
        self.pipeline
            .use_shader(FRAGMENT_SHADER_BIT, self.fragment_shader.id);
        self.pipeline.bind();
        self.vertex_array.draw(TRIANGLE_FAN);
        self.fbo.unbind(app);

        if let Some(target) = target {
            target.bind();
        }

        unsafe {
            (app.extensions.gl_program_uniform_1ui_arb)(
                self.display_shader.id,
                self.texture_loc,
                self.fbo.texture.handle,
            );
        }
        self.pipeline
            .use_shader(FRAGMENT_SHADER_BIT, self.display_shader.id);
        self.pipeline.bind();
        self.vertex_array.draw(TRIANGLE_FAN);

        if let Some(target) = target {
            target.unbind(&app);
        }

        Ok(())
    }

    fn resize(&mut self, size: PhysicalSize<u32>) -> Result<()> {
        self.fbo.resize(
            (size.width as f32 / self.scale) as i32,
            (size.height as f32 / self.scale) as i32,
        )
    }
}
