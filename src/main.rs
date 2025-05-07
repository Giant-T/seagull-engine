use lib::app::AppState;
use winit::event_loop::{ControlFlow, EventLoop};

mod lib;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app_state = AppState::default();
    event_loop.run_app(&mut app_state).unwrap();
}
