use std::sync::{Arc, Mutex};

use anyhow::Result;
use egui::{CentralPanel, Id, SidePanel};
use egui_glow::Painter;
use glm::{Vec2, vec2};
use glow::{HasContext, SCISSOR_TEST};
use log::{error, info};
use rand::Rng;
use seagull_lib::app::{AppContext, HandleApp};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::voronoi::{self, Voronoi};

pub struct AppHandler {
    voronoi: Voronoi,
    egui_state: egui_winit::State,
    egui_painter: Painter,
}

fn generate_random_vec2s() -> Vec<Vec2> {
    let mut rng = rand::rng();
    (0..16)
        .map(|_| vec2(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0)))
        .collect()
}

impl AppHandler {
    pub fn new(gl: Arc<glow::Context>, window: &Window, size: &PhysicalSize<u32>) -> Result<Self> {
        let voronoi = Voronoi::new(gl.clone(), generate_random_vec2s())?;
        let egui_painter = Painter::new(gl.clone(), "", None, true)?;
        let egui_context = egui::Context::default();
        let egui_state = egui_winit::State::new(
            egui_context,
            egui::ViewportId::ROOT,
            window,
            None,
            None,
            None,
        );

        Ok(Self {
            voronoi,
            egui_state,
            egui_painter,
        })
    }
}

impl HandleApp for AppHandler {
    fn render(&mut self, context: &AppContext) -> Result<()> {
        let size = [context.size.width, context.size.height];
        self.voronoi.apply(0, 0, size[0] as i32, size[1] as i32)?;

        let input = self.egui_state.take_egui_input(context.get_window());
        let full_output = self.egui_state.egui_ctx().run(input, |ctx| {
            SidePanel::left(Id::new("SidePanel"))
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label("Hello, World!");
                    ui.allocate_space(ui.available_size());
                });
        });

        self.egui_state
            .handle_platform_output(context.get_window(), full_output.platform_output);
        let paint_jobs = self.egui_state.egui_ctx().tessellate(
            full_output.shapes,
            self.egui_state.egui_ctx().pixels_per_point(),
        );

        self.egui_painter.paint_and_update_textures(
            size,
            self.egui_state.egui_ctx().pixels_per_point(),
            &paint_jobs,
            &full_output.textures_delta,
        );

        Ok(())
    }

    fn event(&mut self, window: &Window, event: &WindowEvent) {
        if self.egui_state.on_window_event(window, event).consumed {
            return;
        }
    }

    fn resize(&mut self, _size: &PhysicalSize<u32>) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, context: &AppContext) -> Result<()> {
        info!("delta time: {}s", context.get_delta_time());

        Ok(())
    }

    // TODO: Find out what i should do
    fn handle_error(&self, error: anyhow::Error) {
        error!("{error:?}");
        panic!();
    }
}

impl Drop for AppHandler {
    fn drop(&mut self) {
        self.egui_painter.destroy();
    }
}
