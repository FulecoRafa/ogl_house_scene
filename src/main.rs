#[macro_use]
extern crate glium;
extern crate image;

use std::{cmp::max, include_bytes};
use std::io::empty;

use glium::{Display, Surface};
use glium::backend::glutin::DisplayCreationError;
use glium::glutin;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::texture::SrgbTexture2d;

use model::generic_model::GenericModel;

use crate::assets::{
    load_tex,
    transform::Transform,
    vertex::Light,
};
use crate::assets::vertex::{Normal, Vertex};
use crate::event_handler::EventHandler;
use crate::model::{get_light, get_light_rotation, Model, set_light, set_light_rotation, set_program};

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

    let bus = GenericModel::from_obj(&display, "models/bus.obj".to_string());
    let dragon = GenericModel::from_obj(&display, "models/Dragon.obj".to_string());
    let gas_station = GenericModel::from_obj(&display, "models/Station.obj".to_string());
    let dennis = GenericModel::from_obj(&display, "models/rp_dennis_posed_004_30k.OBJ".to_string());
    let fabienne_percy = GenericModel::from_obj(&display, "models/rp_fabienne_percy_posed_001_60k.obj".to_string());
    let altair = GenericModel::from_obj(&display, "models/assassins-creed-altair.obj".to_string());
    let railgun = GenericModel::from_obj(&display, "models/Railgun_Prototype-Wavefront OBJ.obj".to_string());

    let ground1_vertices:Vec<Vertex> = vec![
        [-0.38, 0.0, -10.0],
        [-0.38, 0.0, 10.0],
        [-10.0, 0.0, 10.0],
        [-10.0, 0.0, -10.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [
                    if v[2] == -10.0 { 0.0 } else { 1.0 },
                    if v[0] == -10.0 { 0.0 } else { 1.0 },
                ]
            }
        })
        .collect();

    let ground2_vertices:Vec<Vertex> = vec![
        [0.98, 0.0, -10.0],
        [0.98, 0.0, 10.0],
        [10.0, 0.0, 10.0],
        [10.0, 0.0, -10.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [
                    if v[2] == -10.0 { 0.0 } else { 1.0 },
                    if v[0] == 0.98 { 0.0 } else { 1.0 },
                ]
            }
        })
        .collect();

    let road_vertices:Vec<Vertex> = vec![
        [-0.38, 0.0, -10.0],
        [-0.38, 0.0, 10.0],
        [0.98, 0.0, 10.0],
        [0.98, 0.0, -10.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [
                    if v[2] == -10.0 { 0.0 } else { 1.0 },
                    if v[0] == -0.38 { 0.0 } else { 1.0 },
                ]
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
    let ground1 = GenericModel::new(&display, &ground1_vertices, &vec![0, 1, 2, 2, 3, 0], &ground_normals);
    let ground2 = GenericModel::new(&display, &ground2_vertices, &vec![0, 1, 2, 2, 3, 0], &ground_normals);
    let road = GenericModel::new(&display, &road_vertices, &vec![0, 1, 2, 2, 3, 0], &ground_normals);

    let mut event_handler = EventHandler{
        grow: 0.1,
        ..Default::default()
    };

    let bus_pos = (0.1, 0.01, 0.1);
    let dragon_pos = (0.6, 0.55, -0.1);
    let gas_station_pos = (0.1, 0.0, 0.0);
    let dennis_pos = (-0.22, 0.0, 0.3);
    let fabienne_pos = (-0.12, 0.0, 0.3);
    let altair_pos = (-0.71, 0.0, -1.01);
    let railgun_pos = (-0.31, 0.07, -1.02);

    let fabienne_tex = load_tex!(&display, "../textures/rp_fabienne_percy_posed_001_dif_2k.jpg", jpg);
    let dennis_tex = load_tex!(&display, "../textures/rp_dennis_posed_004_dif_2k.jpg", jpg);
    let altair_tex = load_tex!(&display, "../textures/kaleidoscope.jpg", jpg);
    let ground_tex = load_tex!(&display, "../textures/ground.jpg", jpg);
    let dragon_tex = load_tex!(&display, "../textures/Dragon_ground_color.jpg", jpg);
    let station_tex = load_tex!(&display, "../textures/gasstation red.png", png);
    let bus_tex = load_tex!(&display, "../textures/bus_d.png", png);
    let railgun_tex = load_tex!(&display, "../textures/Railgun_color.jpg", jpg);
    let road_tex = load_tex!(&display, "../textures/road.jpg", jpg);

    let mut dragon_spin_self = 0.0f32;
    let mut dragon_spin_around = 0.0f32;
    let mut bus_translate_z = 0.0f32;
    let mut dennis_translate_x = 0.0f32;
    let mut fabienne_translate_x = 0.0f32;
    let mut railgun_spin_self = 0.0f32;
    let mut altair_spin_self = 0.0f32;

    event_loop.run(move |event, _, control_flow| {
        let mut target = display.draw();
        target.clear_color_and_depth((0., 0., 1., 1.), 1.);

        set_wait(control_flow, 16_666_667);

        event_handler.handle_event(event, control_flow);
        dragon_spin_self += 0.001;
        dragon_spin_around += 0.01;
        bus_translate_z += if bus_pos.2 + bus_translate_z < 2.2 {0.0001} else {0.0};
        fabienne_translate_x -= if fabienne_translate_x + fabienne_pos.0 > -1.9 {0.00005} else {0.0};
        dennis_translate_x -= if dennis_translate_x + dennis_pos.0 > -2.2 {0.000075} else {0.0};
        railgun_spin_self += 0.01;
        altair_spin_self -= 0.01;

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
        set_light_rotation(get_light_rotation() + 0.02);

        bus.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotate_self: [spin, tilt, 0.],
                scale: 5.0,
                translation: [bus_pos.0, bus_pos.1, bus_pos.2 + bus_translate_z],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                texture: Some(&bus_tex),
                ..Default::default()
            }
        );

        dragon.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotation: [0., dragon_spin_around, 0.],
                rotate_self: [spin, dragon_spin_self, 0.],
                scale: 2.4,
                translation: [dragon_pos.0, dragon_pos.1, dragon_pos.2],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                texture: Some(&dragon_tex),
                ..Default::default()
            }
        );

        gas_station.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotate_self: [spin, tilt, 0.],
                scale: 17.5,
                translation: [translate_x + gas_station_pos.0, translate_y + gas_station_pos.1, 0. + gas_station_pos.2],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                texture: Some(&station_tex),
                ..Default::default()
            }
        );

        dennis.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotate_self: [spin, 3.0, 0.],
                scale: 0.13,
                translation: [dennis_translate_x + dennis_pos.0, dennis_pos.1, dennis_pos.2],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                texture: Some(&dennis_tex),
                ..Default::default()
            }
        );

        fabienne_percy.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotate_self: [spin, 3.0, 0.],
                scale: 0.13,
                translation: [fabienne_translate_x + fabienne_pos.0, fabienne_pos.1, fabienne_pos.2],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                texture: Some(&fabienne_tex),
                ..Default::default()
            }
        );

        altair.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotate_self: [spin, altair_spin_self, 0.],
                scale: 0.30,
                translation: [altair_pos.0, altair_pos.1, altair_pos.2],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                texture: Some(&altair_tex),
                ..Default::default()
            }
        );

        railgun.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotate_self: [spin, railgun_spin_self, 0.],
                scale: 30.17,
                translation: [railgun_pos.0, railgun_pos.1, railgun_pos.2],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                texture: Some(&railgun_tex),
                ..Default::default()
            }
        );

        ground1.draw(
            &mut target,
            &draw_params,
            &Transform {
                texture: Some(&ground_tex),
                frame_dimensions: Some(dimensions),
                view: [position, direction, up],
                ..Default::default()
            }
        );

        ground2.draw(
            &mut target,
            &draw_params,
            &Transform {
                texture: Some(&ground_tex),
                frame_dimensions: Some(dimensions),
                view: [position, direction, up],
                ..Default::default()
            }
        );

        road.draw(
            &mut target,
            &draw_params,
            &Transform {
                texture: Some(&road_tex),
                frame_dimensions: Some(dimensions),
                view: [position, direction, up],
                ..Default::default()
            }
        );

        target.finish().unwrap();
    });
}

/// Defines the wait time for the next frame
fn set_wait(cf: &mut ControlFlow, nanos: u64) {
    let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(nanos);
    *cf = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
}