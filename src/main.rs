use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const SCALE: f32 = 2.0;

const TITLE: &str = "Falling Sand Simulator";

#[derive(Default)]
struct FallingApp {
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for FallingApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH as f32 * SCALE, HEIGHT as f32 * SCALE);
        let attribs = Window::default_attributes()
            .with_title(TITLE)
            .with_inner_size(size);
        let window = Arc::new(event_loop.create_window(attribs).unwrap());
        self.window = Some(window);
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let mut app = FallingApp::default();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}
