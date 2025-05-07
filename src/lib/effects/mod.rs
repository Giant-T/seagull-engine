use anyhow::Result;

use super::frame_buffer_object::FrameBufferObject;

pub mod pixelate;

pub trait Effect {
    fn apply(&self, source: &FrameBufferObject, target: Option<&FrameBufferObject>) -> Result<()>;

    fn resize(&mut self, width: i32, height: i32) -> Result<()>;
}
