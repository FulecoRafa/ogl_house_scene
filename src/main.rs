#[macro_use]
extern crate glium;

use glium::{Display, Surface};
use glium::backend::glutin::DisplayCreationError;
use glium::glutin;
use glium::glutin::event_loop::{ControlFlow, EventLoop};

use model::generic_model::GenericModel;

use crate::assets::{
    transform::Transform,
    vertex::Light,
};
use crate::assets::vertex::{Normal, Vertex};
use crate::event_handler::EventHandler;
use crate::model::{Model, set_program};

mod model;
mod assets;
mod event_handler;

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
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
        ..Default::default()
    };

    let humvee = GenericModel::from_obj(&display, "models/Humvee.obj".to_string());
    let dragon = GenericModel::from_obj(&display, "models/Dragon.obj".to_string());
    let gas_station = GenericModel::from_obj(&display, "models/Station.obj".to_string());
    let ground_vertices:Vec<Vertex> = vec![
        [-1.0, 0.0, -1.0],
        [1.0, 0.0, -1.0],
        [1.0, 0.0, 1.0],
        [-1.0, 0.0, 1.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [0.0, 0.0]
            }
        })
        .collect();
    let ground_normals: Vec<Normal> = vec![[0.0, 1.0, 0.0]; 3]
        .iter()
        .map(|n| {
            Normal {
                normal: [n[0], n[1], n[2]]
            }
        })
        .collect();
    let ground = GenericModel::new(&display, &ground_vertices, &vec![0, 1, 2, 1, 2, 3], &ground_normals);


    let mut event_handler = EventHandler{
        grow: 0.1,
        ..Default::default()
    };

    let humvee_pos = (0.1, 0.0, 0.1);
    let dragon_pos = (-0.1, 0.3, -0.1);
    let gas_station_pos = (0.1, 0.0, 0.0);


    event_loop.run(move |event, _, control_flow| {
        let mut target = display.draw();
        target.clear_color_and_depth((0., 0., 1., 1.), 1.);

        set_wait(control_flow, 16_666_667);

        event_handler.handle_event(event, control_flow);

        let EventHandler {
            grow,
            tilt,
            spin,
            translate_x,
            translate_y,
            direction,
            position,
            up
        } = event_handler;

        let dimensions = target.get_dimensions();

        humvee.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotate_self: [spin, tilt, 0.],
                scale: 0.2,
                translation: [translate_x + humvee_pos.0, translate_y + humvee_pos.1, 0. + humvee_pos.2],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                ..Default::default()
            }
        );

        dragon.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotate_self: [spin, tilt, 0.],
                scale: 1.4,
                translation: [translate_x + dragon_pos.0, translate_y + dragon_pos.1, 0. + dragon_pos.2],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                ..Default::default()
            }
        );

        gas_station.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotate_self: [spin, tilt, 0.],
                scale: 10.5,
                translation: [translate_x + gas_station_pos.0, translate_y + gas_station_pos.1, 0. + gas_station_pos.2],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                ..Default::default()
            }
        );

        ground.draw(
            &mut target,
            &draw_params,
            &Transform::default()
        );


        target.finish().unwrap();
    });
}

/// Defines the wait time for the next frame
fn set_wait(cf: &mut ControlFlow, nanos: u64) {
    let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(nanos);
    *cf = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
}