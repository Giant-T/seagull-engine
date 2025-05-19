use std::rc::Rc;

use anyhow::Result;
use seagull_lib::{app::HandleApp, effects::Effect};
use winit::dpi::PhysicalSize;

use crate::pixelate::Pixelate;

pub struct AppHandler {
    pixelate: Pixelate,
}

impl AppHandler {
    pub fn new(
        extensions: Rc<seagull_lib::extensions::Extensions>,
        size: &PhysicalSize<u32>,
    ) -> Result<Self> {
        let pixelate = Pixelate::new(extensions, size, 2.0)?;

        Ok(Self { pixelate })
    }
}

impl HandleApp for AppHandler {
    fn render(&self, context: &seagull_lib::app::AppContext) -> Result<()> {
        self.pixelate.apply(context, None, None)?;

        Ok(())
    }

    fn resize(&mut self, size: &winit::dpi::PhysicalSize<u32>) -> Result<()> {
        self.pixelate.resize(size)?;

        Ok(())
    }
}
