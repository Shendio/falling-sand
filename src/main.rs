use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
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
    pixels: Option<Pixels<'static>>,
}

impl ApplicationHandler for FallingApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH as f32 * SCALE, HEIGHT as f32 * SCALE);
        let attribs = Window::default_attributes()
            .with_title(TITLE)
            .with_inner_size(size);
        let window = Arc::new(
            event_loop
                .create_window(attribs)
                .expect("ERROR: Failed to create a window."),
        );
        self.window = Some(window.clone());

        let inner_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(inner_size.width, inner_size.height, window);
        let pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)
            .expect("ERROR: Failed to create a pixel buffer.");

        self.pixels = Some(pixels);
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
                if let (Some(window), Some(pixels)) = (self.window.as_ref(), self.pixels.as_mut()) {
                    let frame = pixels.frame_mut();

                    for pixel in frame.chunks_exact_mut(4) {
                        pixel.copy_from_slice(&[0x15, 0x15, 0x15, 0xFF]);
                    }

                    if let Err(e) = pixels.render() {
                        println!("ERROR: Failed to render pixels: {:?}", e);
                        event_loop.exit();
                    }
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
