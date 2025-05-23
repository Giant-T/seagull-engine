use std::sync::Arc;

use anyhow::Result;
use glow::{
    CLAMP_TO_EDGE, HasContext, NEAREST, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER,
    TEXTURE_WRAP_S, TEXTURE_WRAP_T,
};
use log::info;

// TODO: create an enum for texture formats

pub struct Texture {
    pub id: glow::Texture,
    format: u32,
    gl: Arc<glow::Context>,
}

impl Texture {
    pub fn new(gl: Arc<glow::Context>, width: i32, height: i32, format: u32) -> Result<Self> {
        let id = unsafe {
            gl.create_named_texture(TEXTURE_2D)
                .or_else(|s| Err(anyhow::anyhow!(s)))?
        };
        unsafe {
            gl.texture_storage_2d(id, 1, format, width, height);
            gl.texture_parameter_i32(id, TEXTURE_MIN_FILTER, NEAREST as i32);
            gl.texture_parameter_i32(id, TEXTURE_MAG_FILTER, NEAREST as i32);
            gl.texture_parameter_i32(id, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
            gl.texture_parameter_i32(id, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
        }

        info!("Initialized texture {id:?}");

        Ok(Self { id, format, gl })
    }

    pub fn resize(&mut self, width: i32, height: i32) -> Result<()> {
        unsafe {
            self.gl.delete_texture(self.id);

            self.id = self
                .gl
                .create_named_texture(TEXTURE_2D)
                .or_else(|s| Err(anyhow::anyhow!(s)))?;
            self.gl
                .texture_storage_2d(self.id, 1, self.format, width, height);
            self.gl
                .texture_parameter_i32(self.id, TEXTURE_MIN_FILTER, NEAREST as i32);
            self.gl
                .texture_parameter_i32(self.id, TEXTURE_MAG_FILTER, NEAREST as i32);
            self.gl
                .texture_parameter_i32(self.id, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
            self.gl
                .texture_parameter_i32(self.id, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
        }

        info!("Resized texture {:?}", self.id);

        Ok(())
    }

    pub fn activate_texture(&self, unit: u32) {
        unsafe {
            self.gl.active_texture(unit);
            self.gl.bind_texture(TEXTURE_2D, Some(self.id));
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.id);
        }
    }
}
