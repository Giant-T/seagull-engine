use anyhow::Result;
use winit::dpi::PhysicalSize;

use super::{app::App, frame_buffer_object::FrameBufferObject};

pub mod pixelate;

pub trait Effect {
    fn apply(
        &self,
        app: &App,
        source: Option<&FrameBufferObject>,
        target: Option<&FrameBufferObject>,
    ) -> Result<()>;

    fn resize(&mut self, size: PhysicalSize<u32>) -> Result<()>;
}
