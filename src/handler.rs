use std::rc::Rc;

use anyhow::Result;
use log::{error, info};
use seagull_lib::{
    app::{AppContext, HandleApp},
    effects::Effect,
};
use winit::dpi::PhysicalSize;

use crate::pixelate::Pixelate;

pub struct AppHandler {
    pixelate: Pixelate,
}

impl AppHandler {
    pub fn new(gl: Rc<glow::Context>, size: &PhysicalSize<u32>) -> Result<Self> {
        let pixelate = Pixelate::new(gl.clone(), size, 2.0)?;

        Ok(Self { pixelate })
    }
}

impl HandleApp for AppHandler {
    fn render(&self, context: &AppContext) -> Result<()> {
        self.pixelate.apply(context, None, None)?;

        Ok(())
    }

    fn resize(&mut self, size: &PhysicalSize<u32>) -> Result<()> {
        self.pixelate.resize(size)?;

        Ok(())
    }

    fn update(&mut self, context: &AppContext) -> Result<()> {
        // info!("delta time: {}s", context.get_delta_time());

        Ok(())
    }

    // TODO: Find out what i should do
    fn handle_error(&self, error: anyhow::Error) {
        error!("{error:?}");
        panic!();
    }
}
