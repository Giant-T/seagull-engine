use std::sync::{Arc, Mutex};

use anyhow::Result;
use egui::{CentralPanel, Id, SidePanel};
use egui_glow::Painter;
use glow::{HasContext, SCISSOR_TEST};
use log::{error, info};
use seagull_lib::app::{AppContext, HandleApp};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::pixelate::Pixelate;

pub struct AppHandler {
    pixelate: Arc<Mutex<Pixelate>>,
    egui_state: egui_winit::State,
    egui_painter: Painter,
}

impl AppHandler {
    pub fn new(gl: Arc<glow::Context>, window: &Window, size: &PhysicalSize<u32>) -> Result<Self> {
        let pixelate = Arc::new(Mutex::new(Pixelate::new(gl.clone(), size, 2.0)?));
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
            pixelate,
            egui_state,
            egui_painter,
        })
    }
}

impl HandleApp for AppHandler {
    fn render(&mut self, context: &AppContext) -> Result<()> {
        let size = [context.size.width, context.size.height];

        let input = self.egui_state.take_egui_input(context.get_window());
        let full_output = self.egui_state.egui_ctx().run(input, |ctx| {
            SidePanel::left(Id::new("SidePanel"))
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label("Hello, World!");
                    ui.allocate_space(ui.available_size());
                });

            CentralPanel::default().show(ctx, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::Vec2::new(ui.available_width(), ui.available_height()),
                        egui::Sense::hover(),
                    );

                    let pixelate = self.pixelate.clone();
                    let callback = egui::PaintCallback {
                        rect,
                        callback: Arc::new(egui_glow::CallbackFn::new(move |info, painter| {
                            let viewport = info.viewport_in_pixels();
                            unsafe {
                                painter.gl().disable(SCISSOR_TEST); // Clean up
                            }
                            let _ = pixelate.lock().unwrap().apply(
                                viewport.left_px,
                                viewport.top_px,
                                viewport.width_px,
                                viewport.height_px,
                                None,
                                None,
                            );
                        })),
                    };

                    ui.painter().add(callback)
                });
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

    fn resize(&mut self, size: &PhysicalSize<u32>) -> Result<()> {
        self.pixelate.lock().unwrap().resize(size)?;

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
