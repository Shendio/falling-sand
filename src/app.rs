use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, window::Window,
};

use crate::world::{Particle, World};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const SCALE: f32 = 2.0;

const TITLE: &str = "Falling Sand Simulator";

const BACKGROUND_COLOR: [u8; 4] = [0x15, 0x15, 0x15, 0xFF];
const SAND_COLOR: [u8; 4] = [0xFA, 0xE8, 0xB4, 0xFF];
const WATER_COLOR: [u8; 4] = [0x44, 0x88, 0xFF, 0xFF];

#[derive(Default)]
pub struct FallingApp {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    world: World,
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

        self.world = World::new(WIDTH as usize, HEIGHT as usize);
    }
    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(window) = &self.window {
            self.world.update();
            window.request_redraw();
        }
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
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();
                    let particles = self.world.particles();

                    for (pixel, particle) in frame.chunks_exact_mut(4).zip(particles.iter()) {
                        let color = match particle {
                            Particle::Sand => SAND_COLOR,
                            Particle::Water => WATER_COLOR,
                            Particle::Air => BACKGROUND_COLOR,
                        };

                        pixel.copy_from_slice(&color);
                    }

                    if let Err(e) = pixels.render() {
                        println!("ERROR: Failed to render pixels: {:?}", e);
                        event_loop.exit();
                    }
                }
            }
            _ => (),
        }
    }
}
