use anyhow::Result;
use winit::dpi::PhysicalSize;

use super::{app::App, frame_buffer::FrameBuffer};

pub mod pixelate;

pub trait Effect {
    fn apply(
        &self,
        app: &App,
        source: Option<&FrameBuffer>,
        target: Option<&FrameBuffer>,
    ) -> Result<()>;

    fn resize(&mut self, size: PhysicalSize<u32>) -> Result<()>;
}
