//~ mod mon;
mod draw;

use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::{Event, EventsLoop, WindowBuilder, WindowEvent};
//~ use winit::{ControlFlow, VirtualKeyCode};

use image;

const SCREEN_WIDTH: usize = 192;
const SCREEN_HEIGHT: usize = 144;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
    player_image: image::DynamicImage,
    enemy_image: image::DynamicImage,
    basic_font: image::DynamicImage,
    //~ player_mon: mon::Mon,
    //~ enemy_mon: mon::Mon,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut events_loop = EventsLoop::new();
    //~ let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Fruitmon")
            .with_dimensions(size)
            .with_min_dimensions(size)
            .build(&events_loop)?
    };
    let mut hidpi_factor = window.get_hidpi_factor();

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture =
            SurfaceTexture::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)?
    };
    let mut world = World::new();

    //~ let mut frames = 0;

    let mut quit = false;
    while !quit {
        /*TODO: Make a constant update rate*/
        {
            world.update();
            world.draw(pixels.get_frame());
            pixels.render();
            //~ frames = frames + 1;
        }

        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: inner_event, ..
            } => {
                use WindowEvent::*;
                match inner_event {
                    CloseRequested => {
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
        let player_image = draw::load_image("assets/testmon_back.png").unwrap();
        let enemy_image = draw::load_image("assets/testmon.png").unwrap();
        let basic_font = draw::load_image("assets/test_font.png").unwrap();
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 3,
            player_image,
            enemy_image,
            basic_font,
            //~ player_mon: mon::Mon::new(),
            //~ enemy_mon: mon::Mon::new(),
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE - 1 >= SCREEN_WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE - 1 >= SCREEN_HEIGHT as i16 {
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
            let x = (i % SCREEN_WIDTH) as i16;
            let y = (i / SCREEN_WIDTH) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x00, 0x00, 0x00, 0xff]
            } else {
                [0x28, 0x68, 0x40, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
        draw::blit(frame, (0, 64), &self.player_image);
        draw::blit(frame, (128, 0), &self.enemy_image);
    }
}
