use anyhow::Result;
use winit::dpi::PhysicalSize;

use super::{app::AppContext, frame_buffer::FrameBuffer};

pub trait Effect {
    fn apply(
        &self,
        context: &AppContext,
        source: Option<&FrameBuffer>,
        target: Option<&FrameBuffer>,
    ) -> Result<()>;

    fn resize(&mut self, size: &PhysicalSize<u32>) -> Result<()>;
}
