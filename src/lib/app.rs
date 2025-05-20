use std::{
    ffi::{CStr, CString},
    num::NonZeroU32,
    process::exit,
    rc::Rc,
};

use anyhow::{Context, Error, Result};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use log::info;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    raw_window_handle::HasWindowHandle,
    window::Window,
};

use super::{
    extensions::Extensions,
    gl::{self, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT},
};

pub trait HandleApp {
    fn update(&mut self, context: &AppContext) -> Result<()>;
    fn render(&self, context: &AppContext) -> Result<()>;
    fn resize(&mut self, size: &PhysicalSize<u32>) -> Result<()>;
    fn handle_error(&self, error: Error);
}

type HandlerCreator =
    fn(extensions: Rc<Extensions>, size: &PhysicalSize<u32>) -> Result<Box<dyn HandleApp>>;

pub struct AppContext {
    window: Window,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
    pub size: PhysicalSize<u32>,
    pub extensions: Rc<Extensions>,
}

struct Runtime {
    context: AppContext,
    handler: Box<dyn HandleApp>,
}

impl Runtime {
    fn new(event_loop: &ActiveEventLoop, handler_creator: HandlerCreator) -> Result<Self> {
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
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
        }

        info!("Initialized the window");

        return Ok(Self {
            handler: handler_creator(extensions.clone(), &size)?,
            context: AppContext {
                window,
                surface,
                context,
                size,
                extensions,
            },
        });
    }

    fn render(&mut self) -> Result<()> {
        self.handler.update(&self.context)?;

        unsafe {
            gl::Clear(DEPTH_BUFFER_BIT | COLOR_BUFFER_BIT);
        }

        self.handler.render(&self.context)?;

        self.context.surface.swap_buffers(&self.context.context)?;
        Ok(())
    }

    fn resize(&mut self, size: PhysicalSize<u32>) -> Result<()> {
        self.context.size = size;

        self.handler.resize(&size)?;

        Ok(())
    }
}

pub struct App {
    state: AppState,
}

impl App {
    pub fn new(handler_creator: HandlerCreator) -> Self {
        let state = AppState::Uninitialized(handler_creator);
        return Self { state };
    }

    pub fn run(&mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(&mut self.state)?;

        Ok(())
    }
}

enum AppState {
    Uninitialized(HandlerCreator),
    Initialized(Runtime),
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Self::Uninitialized(handler_creator) = self {
            let app = Runtime::new(event_loop, *handler_creator);

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
                    if let Err(err) = app.render() {
                        // TODO: Handle errors correctly
                        app.handler.handle_error(err);
                    }
                    app.context.window.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                if let Self::Initialized(app) = self {
                    if let Err(err) = app.resize(size) {
                        // TODO: Handle errors correctly
                        app.handler.handle_error(err);
                    }
                }
            }
            _ => {}
        }
    }
}
