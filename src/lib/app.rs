use std::{
    ffi::{CStr, CString},
    num::NonZeroU32,
    process::exit,
    rc::Rc,
};

use anyhow::{Context, Result};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent,
    event_loop::ActiveEventLoop, raw_window_handle::HasWindowHandle, window::Window,
};

use super::{effects::Effect, extensions::Extensions, gl::{self, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT}};

pub struct App {
    window: Window,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
    pub size: PhysicalSize<u32>,
    extensions: Rc<Extensions>,
    effects: Vec<Box<dyn Effect>>,
}

impl App {
    fn new(event_loop: &ActiveEventLoop) -> Result<Self> {
        let attributes = Window::default_attributes().with_title("Rust Playground");

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_depth_size(24)
            .with_stencil_size(8);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(attributes));

        let (window, config) = display_builder
            .build(event_loop, template, |configs| {
                configs.reduce(|a, _b| a).unwrap()
            })
            .expect("Failed to create window and config");

        let window = window.expect("Window creation failed");
        let window_handle = window.window_handle()?;
        let display = config.display();

        let size: winit::dpi::PhysicalSize<u32> = window.inner_size();
        let width = NonZeroU32::new(size.width).context("Unable to convert width to non zero")?;
        let height =
            NonZeroU32::new(size.height).context("Unable to convert height to non zero")?;
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window_handle.as_raw(),
            width,
            height,
        );

        let surface = unsafe {
            display
                .create_window_surface(&config, &attrs)
                .expect("Failed to create window surface")
        };

        let context_attrs = ContextAttributesBuilder::new()
            .with_debug(true)
            .build(Some(window_handle.as_raw()));
        let not_current = unsafe { display.create_context(&config, &context_attrs)? };

        let context = not_current
            .make_current(&surface)
            .context("Failed to make context current")?;

        let extensions;
        unsafe {
            let get_proc_address = |s: &CStr| display.get_proc_address(s) as *const _;
            gl::load_with(|s| get_proc_address(CString::new(s).unwrap().as_c_str()));
            extensions = Rc::new(Extensions::load_extensions(get_proc_address)?);
            gl::Viewport(0, 0, size.width as i32, size.height as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        return Ok(Self {
            window,
            surface,
            context,
            size,
            effects: vec![],
            extensions,
        });
    }

    fn render(&self) -> Result<()> {
        unsafe {
            gl::Clear(DEPTH_BUFFER_BIT | COLOR_BUFFER_BIT);
        }
        self.surface.swap_buffers(&self.context)?;
        // TODO: Handle errors correctly

        Ok(())
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
    }
}

pub enum AppState {
    Uninitialized,
    Initialized(App),
}

impl Default for AppState {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Self::Uninitialized = self {
            let app = App::new(event_loop);

            if let Ok(app) = app {
                *self = Self::Initialized(app);
            }
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                exit(0);
            }
            WindowEvent::RedrawRequested => {
                if let Self::Initialized(app) = self {
                    app.render().unwrap(); // TODO: Handle errors correctly
                }
            }
            WindowEvent::Resized(size) => {
                if let Self::Initialized(app) = self {
                    app.resize(size);
                }
            }
            _ => {}
        }
    }
}
