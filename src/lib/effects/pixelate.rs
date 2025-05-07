use std::{
    ffi::{CString, c_char},
    rc::Rc,
};

use anyhow::Result;
use winit::dpi::PhysicalSize;

use crate::lib::{
    extensions::Extensions,
    frame_buffer_object::FrameBufferObject,
    gl::{
        self, FALSE, FRAGMENT_SHADER, FRAGMENT_SHADER_BIT, LINK_STATUS, VALIDATE_STATUS,
        VERTEX_SHADER, VERTEX_SHADER_BIT,
    },
};

use super::Effect;

pub struct Pixelate {
    fbo: FrameBufferObject,
    scale: f32,
    vertex_shader: u32,
    fragment_shader: u32,
    pipeline: u32,
    texture_loc: i32,
}

impl Pixelate {
    pub fn new(extensions: Rc<Extensions>, size: &PhysicalSize<u32>, scale: f32) -> Result<Self> {
        let fbo = FrameBufferObject::new(
            extensions,
            (size.width as f32 * scale) as i32,
            (size.height as f32 * scale) as i32,
        )?;

        let vertex_shader;
        unsafe {
            // TODO: should probably put in a function and add a function to check validity
            let str = CString::new(include_str!("VS.glsl"))?;
            let source: [*const c_char; 1] = [str.as_ptr()];
            vertex_shader = gl::CreateShaderProgramv(VERTEX_SHADER, 1, source.as_ptr());

            let mut is_valid: i32 = FALSE.into();
            gl::GetProgramiv(vertex_shader, LINK_STATUS, &mut is_valid as *mut i32);

            if is_valid == FALSE.into() {
                return Err(anyhow::anyhow!("Could not compile vertex shader"));
            }
        }

        let fragment_shader;
        let texture_loc;
        unsafe {
            // TODO: should probably put in a function and add a function to check validity
            let str = CString::new(include_str!("FS.glsl"))?;
            let source: [*const c_char; 1] = [str.as_ptr()];
            fragment_shader = gl::CreateShaderProgramv(FRAGMENT_SHADER, 1, source.as_ptr());

            let mut is_valid: i32 = FALSE.into();
            gl::GetProgramiv(fragment_shader, LINK_STATUS, &mut is_valid as *mut i32);

            if is_valid == FALSE.into() {
                return Err(anyhow::anyhow!("Could not compile fragment shader"));
            }

            texture_loc = gl::GetUniformLocation(fragment_shader, CString::new("FBO")?.as_ptr());
        }

        let mut pipeline = 0;
        unsafe {
            gl::CreateProgramPipelines(1, &mut pipeline);
            gl::UseProgramStages(pipeline, VERTEX_SHADER_BIT, vertex_shader);
            gl::UseProgramStages(pipeline, FRAGMENT_SHADER_BIT, fragment_shader);
            gl::ValidateProgramPipeline(pipeline);

            let mut is_valid: i32 = FALSE.into();
            gl::GetProgramPipelineiv(pipeline, VALIDATE_STATUS, &mut is_valid as *mut i32);
            if is_valid == FALSE.into() {
                return Err(anyhow::anyhow!("Could not link program pipeline"));
            }
        }

        Ok(Self {
            scale,
            fbo,
            vertex_shader,
            fragment_shader,
            texture_loc,
            pipeline,
        })
    }
}

impl Effect for Pixelate {
    fn apply(&self, source: &FrameBufferObject, target: Option<&FrameBufferObject>) -> Result<()> {
        Ok(())
    }

    fn resize(&mut self, width: i32, height: i32) -> Result<()> {
        self.fbo.resize(width, height)
    }
}
