use std::rc::Rc;

use anyhow::Result;
use log::info;

use super::extensions::Extensions;
use super::gl::{
    self, COLOR_ATTACHMENT0, DEPTH_ATTACHMENT, DEPTH_COMPONENT24, FRAMEBUFFER,
    FRAMEBUFFER_COMPLETE, FRAMEBUFFER_INCOMPLETE_ATTACHMENT, FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,
    FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS, FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
    FRAMEBUFFER_INCOMPLETE_MULTISAMPLE, FRAMEBUFFER_INCOMPLETE_READ_BUFFER, FRAMEBUFFER_UNDEFINED,
    FRAMEBUFFER_UNSUPPORTED, RGBA8,
};
use super::{app::AppContext, texture::Texture};

pub struct FrameBuffer {
    id: u32,
    width: i32,
    height: i32,
    pub texture: Texture,
    depth_texture: Texture,
    extensions: Rc<Extensions>,
}

impl FrameBuffer {
    pub fn new(extensions: Rc<Extensions>, width: i32, height: i32) -> Result<Self> {
        let mut id = 0;
        unsafe {
            gl::CreateFramebuffers(1, &mut id);
        }

        let texture = Texture::new(extensions.clone(), width, height, RGBA8)?;
        let depth_texture = Texture::new(extensions.clone(), width, height, DEPTH_COMPONENT24)?;

        unsafe {
            gl::NamedFramebufferTexture(id, COLOR_ATTACHMENT0, texture.id, 0);
            gl::NamedFramebufferTexture(id, DEPTH_ATTACHMENT, depth_texture.id, 0);

            let status = gl::CheckNamedFramebufferStatus(id, FRAMEBUFFER);

            Self::print_frame_buffer_status(status);
            if status != FRAMEBUFFER_COMPLETE {
                return Err(anyhow::anyhow!("FrameBuffer non complete"));
            }
        }

        info!("Initialized frame buffer {id}");

        Ok(Self {
            id,
            width,
            height,
            texture,
            depth_texture,
            extensions,
        })
    }

    ///
    /// Resizes the textures bound to this frame buffer object
    ///
    pub fn resize(&mut self, width: i32, height: i32) -> Result<()> {
        self.height = height;
        self.width = width;

        self.texture.resize(width, height)?;
        self.depth_texture.resize(width, height)?;

        unsafe {
            gl::NamedFramebufferTexture(self.id, COLOR_ATTACHMENT0, self.texture.id, 0);
            gl::NamedFramebufferTexture(self.id, DEPTH_ATTACHMENT, self.depth_texture.id, 0);
        }

        Ok(())
    }

    ///
    /// Binds this frame buffer to be the current rendering target
    ///
    pub fn bind(&self) {
        unsafe {
            gl::Viewport(0, 0, self.width, self.height);
            gl::BindFramebuffer(FRAMEBUFFER, self.id);
        }
    }

    ///
    /// Unbinds this frame buffer to stop being the current rendering target
    ///
    pub fn unbind(&self, app: &AppContext) {
        unsafe {
            gl::Viewport(0, 0, app.size.width as i32, app.size.height as i32);
            gl::BindFramebuffer(FRAMEBUFFER, 0);
        }
    }

    fn print_frame_buffer_status(status: u32) {
        match status {
            FRAMEBUFFER_COMPLETE => info!("Framebuffer complete"),
            FRAMEBUFFER_UNDEFINED => info!("Framebuffer undefined"),
            FRAMEBUFFER_INCOMPLETE_ATTACHMENT => info!("Incomplete attachment"),
            FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => info!("Incomplete attachment"),
            FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => info!("Incomplete draw buffer"),
            FRAMEBUFFER_INCOMPLETE_READ_BUFFER => info!("Incomplete read buffer"),
            FRAMEBUFFER_UNSUPPORTED => info!("Framebuffer unsupported"),
            FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => info!("Incomplete multisample"),
            FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => info!("Incomplete layer targets"),
            _ => info!("Unknown framebuffer error: {status}"),
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}
