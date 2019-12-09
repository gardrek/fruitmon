//~ mod mon;
//~ mod draw;

//~ use image;

use glium::glutin;
use glutin::{Event, EventsLoop, WindowBuilder, ContextBuilder, WindowEvent};
use glutin::dpi::LogicalSize;
use glium::{implement_vertex, uniform};
use glium::Surface;

const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 240;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, texture_coords);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut events_loop = EventsLoop::new();

    let window_size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);

    let wb = WindowBuilder::new()
        .with_dimensions(window_size)
        .with_min_dimensions(window_size)
        //~ .with_vsync(true)
        .with_title("title");

    let cb = ContextBuilder::new()
        .with_depth_buffer(24);

    let display = glium::Display::new(wb, cb, &events_loop)?;

    //~ let mut hidpi_factor = window.get_hidpi_factor();

    let mut triangle_list: Vec<Vertex> = vec![
        Vertex {
            position: [ 0.5, 0.0, 0.0, ],
            normal: [ 0.0, 0.0, 1.0, ],
            texture_coords: [ 0.0, 0.0, ],
        },
        Vertex {
            position: [ -0.5, -0.5, 0.0, ],
            normal: [ 0.0, 0.0, 1.0, ],
            texture_coords: [ 0.0, 0.0, ],
        },
        Vertex {
            position: [ -0.5, 0.5, 0.0, ],
            normal: [ 0.0, 0.0, 1.0, ],
            texture_coords: [ 0.0, 0.0, ],
        },
    ];



    //~ let vertex_buffer = glium::VertexBuffer::dynamic(&display, &triangle_list)?;
    let vertex_buffer = glium::VertexBuffer::new(&display, &triangle_list)?;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = include_str!("vertex.glsl");

    let fragment_shader_src = include_str!("fragment.glsl");

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)?;

    let mut quit = false;

    while !quit {
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_666);

        //~ triangle_list.clear();

        let mut target = display.draw();

        // Draw after this
        target.clear_color_and_depth((0.25, 0.5, 0.75, 1.0), 1.0);

        let render_matrix: [[f32; 4]; 4] = [
            [ 1.0, 0.0, 0.0, 0.0, ],
            [ 0.0, 1.0, 0.0, 0.0, ],
            [ 0.0, 0.0, 1.0, 0.0, ],
            [ 0.0, 0.0, 0.0, 1.0, ],
        ];

        let uniforms = uniform! {
            matrix: Into::<[[f32; 4]; 4]>::into(render_matrix),
            u_light: [1.0f32, 1.0, 1.0],
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },

            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,

            .. Default::default()
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &params)?;

        // Stop drawing here
        target.finish()?;


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
                        //~ world.draw(pixels.get_frame());
                        //~ pixels.render();
                    }
                    /*
                    HiDpiFactorChanged(factor) => {
                        hidpi_factor = factor;
                    }
                    Resized(logical_size) => {
                        let size = logical_size.to_physical(hidpi_factor);
                        let _width = size.width.round() as u32;
                        let _height = size.height.round() as u32;

                        //~ pixels.resize(width, height);
                    }
                    */
                    _ => (),
                }
            }
            _ => (),
        });

        let now = std::time::Instant::now();

        if now > next_frame_time {
            std::thread::sleep(now - next_frame_time);
        }
    }

    Ok(())
}
