#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::{ControlFlow, Event, EventsLoop, VirtualKeyCode, WindowBuilder, WindowEvent};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut events_loop = EventsLoop::new();
    //~ let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_dimensions(size)
            .with_min_dimensions(size)
            .build(&events_loop)?
    };
    let mut hidpi_factor = window.get_hidpi_factor();

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, surface);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();

    let mut frames = 0;

    let mut quit = false;
    while !quit {
        /*TODO: Make a constant update rate*/ {
            world.update();
            world.draw(pixels.get_frame());
            pixels.render();
            println!("{}", frames);
            frames = frames + 1;
        }

        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: inner_event, ..
            } => {
                use WindowEvent::*;
                match inner_event {
                    CloseRequested => {
                        println!("Close Requested (quitting)");
                        quit = true;
                    }
                    Refresh => {
                        world.draw(pixels.get_frame());
                        pixels.render();
                    }
                    HiDpiFactorChanged(factor) => {
                        hidpi_factor = factor;
                    }
                    Resized(logical_size) => {
                        let size = logical_size.to_physical(hidpi_factor);
                        let width = size.width.round() as u32;
                        let height = size.height.round() as u32;

                        pixels.resize(width, height);
                    }
                    _ => (),
                }
            }
            _ => (),
        });
    }

    Ok(())
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE - 1 >= WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE - 1 >= HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
