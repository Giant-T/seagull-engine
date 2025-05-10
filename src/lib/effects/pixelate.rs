use std::rc::Rc;

use anyhow::Result;
use winit::dpi::PhysicalSize;

use crate::lib::{
    app::App,
    extensions::Extensions,
    frame_buffer_object::FrameBufferObject,
    gl::{FRAGMENT_SHADER, VERTEX_SHADER},
    program::{Pipeline, Shader},
};

use super::Effect;

pub struct Pixelate {
    scale: f32,
    vertex_shader: Shader,
    fragment_shader: Shader,
    display_shader: Shader,
    texture_loc: i32,
    pipeline: Pipeline,
}

impl Pixelate {
    pub fn new(extensions: Rc<Extensions>, size: &PhysicalSize<u32>, scale: f32) -> Result<Self> {
        let vertex_shader = Shader::new(VERTEX_SHADER, include_str!("VS.glsl"))?;
        let fragment_shader = Shader::new(FRAGMENT_SHADER, include_str!("FS.glsl"))?;

        let display_shader = Shader::new(FRAGMENT_SHADER, include_str!("Display-FS.glsl"))?;
        let texture_loc = display_shader.get_loc("FBO")?;

        let pipeline = Pipeline::new(&vertex_shader, &fragment_shader)?;

        Ok(Self {
            scale,
            vertex_shader,
            fragment_shader,
            display_shader,
            texture_loc,
            pipeline,
        })
    }
}

impl Effect for Pixelate {
    fn apply(
        &self,
        app: &App,
        _source: Option<&FrameBufferObject>,
        target: Option<&FrameBufferObject>,
    ) -> Result<()> {
        if let Some(target) = target {
            target.bind();
        }

        self.pipeline.bind();
        // TODO:
        // bind vao
        // draw quad

        if let Some(target) = target {
            target.unbind(&app);
        }

        Ok(())
    }

    fn resize(&mut self, size: PhysicalSize<u32>) -> Result<()> {
        Ok(())
    }
}
