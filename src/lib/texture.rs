use std::rc::Rc;

use anyhow::Result;

use super::{
    extensions::Extensions,
    gl::{
        self, CLAMP_TO_EDGE, NEAREST, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER,
        TEXTURE_WRAP_S, TEXTURE_WRAP_T,
    },
};

pub struct Texture {
    pub id: u32,
    handle: u64,
    format: u32,
    extensions: Rc<Extensions>,
}

impl Texture {
    pub fn new(extensions: Rc<Extensions>, width: i32, height: i32, format: u32) -> Result<Self> {
        let mut id = 0;
        let handle;
        unsafe {
            gl::CreateTextures(TEXTURE_2D, 1, &mut id);
            gl::TextureStorage2D(id, 1, format, width, height);
            gl::TextureParameteri(id, TEXTURE_MIN_FILTER, NEAREST as i32);
            gl::TextureParameteri(id, TEXTURE_MAG_FILTER, NEAREST as i32);
            gl::TextureParameteri(id, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
            gl::TextureParameteri(id, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);

            handle = (extensions.gl_get_texture_handle_arb)(id.into());
            (extensions.gl_make_texture_handle_arb)(handle);
        }

        Ok(Self {
            id,
            handle,
            format,
            extensions,
        })
    }

    pub fn resize(&mut self, width: i32, height: i32) -> Result<()> {
        unsafe {
            gl::DeleteTextures(1, &self.id);

            gl::CreateTextures(TEXTURE_2D, 1, &mut self.id);
            gl::TextureStorage2D(self.id, 1, self.format, width, height);
            gl::TextureParameteri(self.id, TEXTURE_MIN_FILTER, NEAREST as i32);
            gl::TextureParameteri(self.id, TEXTURE_MAG_FILTER, NEAREST as i32);
            gl::TextureParameteri(self.id, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
            gl::TextureParameteri(self.id, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);

            self.handle = (self.extensions.gl_get_texture_handle_arb)(self.id.into());
            (self.extensions.gl_make_texture_handle_arb)(self.handle);
        }

        Ok(())
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
