mod app;

use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::FallingApp;

fn main() {
    let mut app = FallingApp::default();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}
