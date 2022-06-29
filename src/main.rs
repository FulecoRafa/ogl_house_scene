#[macro_use]
extern crate glium;

use glium::glutin;
use glium::backend::glutin::DisplayCreationError;
use glium::{Display, Surface};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use crate::model::{Model, set_program};

mod model;
mod assets;

use model::generic_model::GenericModel;
use crate::assets::{
    vertex::Light,
    transform::Transform,
};

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
            range: (0.0, 0.9),
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    let car: GenericModel = GenericModel::new(&display, "models/Humvee.obj".to_string());
    let mut rotate_x: f32 = 0.0;
    let mut rotate_y: f32 = 0.0;

    event_loop.run(move |event, _, control_flow| {
        let mut target = display.draw();
        target.clear_color_and_depth((0., 0., 1., 1.), 1.);

        set_wait(control_flow, 16_666_667);

        rotate_x += 0.1;
        rotate_y += 0.05;

        car.draw(&mut target, &draw_params, &Transform{rotate_self: [rotate_x, rotate_y, 0.0], ..Default::default()});

        target.finish().unwrap();
    });
}

/// Defines the wait time for the next frame
fn set_wait(cf: &mut ControlFlow, nanos: u64) {
    let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(nanos);
    *cf = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
}