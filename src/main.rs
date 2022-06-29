#[macro_use]
extern crate glium;

use glium::{glutin, IndexBuffer, VertexBuffer};
use glium::backend::glutin::DisplayCreationError;
use glium::{Display, Surface};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use crate::model::{Model, set_program};

mod vertex;
mod model;
mod transform;
mod matrices;
mod texture_loader;
mod skybox;

use model::humvee::Humvee;
use vertex::Vertex;
use crate::transform::Transform;
use crate::vertex::Light;

//Starts the window and the event loop
fn start_opengl(
    title: &str,
    mut size: Option<(u32, u32)>,
) -> (EventLoop<()>, Result<Display, DisplayCreationError>) {
    let size = size.get_or_insert((400, 400));

    let event_loop = glutin::event_loop::EventLoop::new();

    let window = glutin::window::WindowBuilder::new()
        .with_title(title)
        .with_inner_size(glutin::dpi::LogicalSize::new(size.0, size.1));

    let context = glutin::ContextBuilder::new().with_depth_buffer(24);

    let display = glium::Display::new(window, context, &event_loop);
    (event_loop, display)
}

fn main() {
    let (event_loop, display) = match start_opengl("First", None) {
        (event_loop, Ok(display)) => (event_loop, display),
        (_, Err(e)) => panic!("Could not create window: {e}"),
    };

    // Instantiates a program source for all models
    set_program(&display);

    // Defining the draw parameters
    let draw_params = glium::draw_parameters::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            range: (0.0, 1.0),
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    // Skybox
    // let skybox = skybox::Skybox::new(&display);
    let square = load_square(&display);
    let texture = load_tex!(display, "./2k_earth_daymap.jpg", jpeg, srgb);
    let square_program = glium::Program::from_source(
        &display,
        "
        #version 140
        in vec3 position;
        in vec2 tex_coords;
        out vec2 v_position;
        void main() {
            v_position = tex_coords;
            gl_Position = vec4(position, 1.0);
        }
        ",
        "
        #version 140
        in vec2 v_position;
        out vec4 f_color;
        uniform sampler2D tex;
        void main() {
            f_color = texture(tex, v_position);
        }
        ",
        None
    ).unwrap();

    // let car_texture = load_tex!(display, "car.png");

    // let car: Humvee = Humvee::new(&display);

    event_loop.run(move |event, _, control_flow| {
        let mut target = display.draw();
        target.clear_color_and_depth((0., 0., 0., 1.), 1.);

        set_wait(control_flow, 16_666_667);

        target.draw(
            &square,
            &glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            &square_program,
            &uniform! {
                tex: &texture
            },
            &draw_params

        ).unwrap();

        // skybox.draw(&mut target, &draw_params, &Transform::default());

        // car.draw(&mut target, &draw_params, &Transform::default());

        target.finish().unwrap();
    });
}

fn load_square(display: &Display) -> VertexBuffer<Vertex> {
    let vertices = [
        Vertex {
            position: [-1., 1., 1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [1., 1., 1.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [-1., -1., 1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [1., -1., 1.0],
            tex_coords: [1.0, 1.0],
        },
    ];
    VertexBuffer::new(display, &vertices).unwrap()
}

/// Defines the wait time for the next frame
fn set_wait(cf: &mut ControlFlow, nanos: u64) {
    let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(nanos);
    *cf = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
}