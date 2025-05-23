use std::sync::Arc;

use anyhow::Result;
use glow::{
    COLOR_ATTACHMENT0, DEPTH_ATTACHMENT, DEPTH_COMPONENT24, FRAMEBUFFER, FRAMEBUFFER_COMPLETE,
    FRAMEBUFFER_INCOMPLETE_ATTACHMENT, FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,
    FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS, FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
    FRAMEBUFFER_INCOMPLETE_MULTISAMPLE, FRAMEBUFFER_INCOMPLETE_READ_BUFFER, FRAMEBUFFER_UNDEFINED,
    FRAMEBUFFER_UNSUPPORTED, HasContext, RGBA8,
};
use log::info;

use crate::texture::Texture;

pub struct FrameBuffer {
    id: glow::Framebuffer,
    width: i32,
    height: i32,
    pub texture: Texture,
    depth_texture: Texture,
    gl: Arc<glow::Context>,
}

impl FrameBuffer {
    pub fn new(gl: Arc<glow::Context>, width: i32, height: i32) -> Result<Self> {
        let id;
        unsafe {
            id = gl
                .create_named_framebuffer()
                .or_else(|s| Err(anyhow::anyhow!(s)))?;
        }

        let texture = Texture::new(gl.clone(), width, height, RGBA8)?;
        let depth_texture = Texture::new(gl.clone(), width, height, DEPTH_COMPONENT24)?;

        unsafe {
            gl.named_framebuffer_texture(Some(id), COLOR_ATTACHMENT0, Some(texture.id), 0);
            gl.named_framebuffer_texture(Some(id), DEPTH_ATTACHMENT, Some(depth_texture.id), 0);

            let status = gl.check_named_framebuffer_status(Some(id), FRAMEBUFFER);

            Self::print_frame_buffer_status(status);
            if status != FRAMEBUFFER_COMPLETE {
                return Err(anyhow::anyhow!("FrameBuffer non complete"));
            }

            gl.bind_framebuffer(FRAMEBUFFER, None);
        }

        info!("Initialized frame buffer {id:?}");

        Ok(Self {
            id,
            width,
            height,
            texture,
            depth_texture,
            gl,
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
            self.gl.named_framebuffer_texture(
                Some(self.id),
                COLOR_ATTACHMENT0,
                Some(self.texture.id),
                0,
            );
            self.gl.named_framebuffer_texture(
                Some(self.id),
                DEPTH_ATTACHMENT,
                Some(self.depth_texture.id),
                0,
            );
        }

        Ok(())
    }

    ///
    /// Binds this frame buffer to be the current rendering target
    ///
    pub fn bind(&self) {
        unsafe {
            self.gl.viewport(0, 0, self.width, self.height);
            self.gl.bind_framebuffer(FRAMEBUFFER, Some(self.id));
        }
    }

    ///
    /// Unbinds this frame buffer to stop being the current rendering target
    ///
    pub fn unbind(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            self.gl
                .viewport(x, y, width, height);
            self.gl.bind_framebuffer(FRAMEBUFFER, None);
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
            self.gl.delete_framebuffer(self.id);
        }
    }
}
