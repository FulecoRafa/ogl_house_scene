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
    let railgun1 = GenericModel::from_obj(&display, "models/Railgun_Prototype-Wavefront OBJ.obj".to_string());
    let railgun2 = GenericModel::from_obj(&display, "models/Railgun_Prototype-Wavefront OBJ.obj".to_string());

    let ground1_vertices:Vec<Vertex> = vec![
        [-0.38, 0.0, -50.0],
        [-0.38, 0.0, 50.0],
        [-50.0, 0.0, 50.0],
        [-50.0, 0.0, -50.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [
                    if v[2] == -50.0 { 0.0 } else { 100.0 },
                    if v[0] == -50.0 { 0.0 } else { 100.0 },
                ]
            }
        })
        .collect();

    let ground2_vertices:Vec<Vertex> = vec![
        [0.98, 0.0, -50.0],
        [0.98, 0.0, 50.0],
        [50.0, 0.0, 50.0],
        [50.0, 0.0, -50.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [
                    if v[2] == -50.0 { 0.0 } else { 100.0 },
                    if v[0] == 0.98 { 0.0 } else { 100.0 },
                ]
            }
        })
        .collect();

    let road_vertices:Vec<Vertex> = vec![
        [-0.38, 0.0, -50.0],
        [-0.38, 0.0, 50.0],
        [0.98, 0.0, 50.0],
        [0.98, 0.0, -50.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [
                    if v[2] == -50.0 { 0.0 } else { 10.0 },
                    if v[0] == -0.38 { 0.0 } else { 1.0 },
                ]
            }
        })
        .collect();

    let skybox_left_vertices: Vec<Vertex> = vec![
        [-50.0, -50.0, -50.0],
        [-50.0, -50.0, 50.0],
        [-50.0, 50.0, 50.0],
        [-50.0, 50.0, -50.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [
                    if v[2] == -50.0 { 0.001 } else { 0.2499 },
                    if v[1] == -50.0 { 0.32999 } else { 0.65999 },
                ]
            }
        })
        .collect();

    // Vertices of the front face of a cube of size 20
    let skybox_front_vertices: Vec<Vertex> = vec![
        [-50.0, -50.0, 50.0],
        [50.0, -50.0, 50.0],
        [50.0, 50.0, 50.0],
        [-50.0, 50.0, 50.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [
                    if v[0] == -50.0 { 0.24999 } else { 0.49999 },
                    if v[1] == -50.0 { 0.32999 } else { 0.65999 },
                ],
            }
        })
        .collect();

    let skybox_top_vertices: Vec<Vertex> = vec![
        [-50.0, 50.0, 50.0],
        [50.0, 50.0, 50.0],
        [50.0, 50.0, -50.0],
        [-50.0, 50.0, -50.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [
                    if v[0] == -50.0 { 0.2499 } else { 0.499 },
                    if v[2] == 50.0 { 0.6599 } else { 0.999 },
                ],
            }
        })
        .collect();

    let skybox_right_vertices: Vec<Vertex> = vec![
        [50.0, -50.0, -50.0],
        [50.0, 50.0, -50.0],
        [50.0, 50.0, 50.0],
        [50.0, -50.0, 50.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [
                    if v[2] == 50.0 { 0.4999 } else { 0.7499 },
                    if v[1] == -50.0 { 0.32999 } else { 0.65999 },
                ],
            }
        })
        .collect();

    let skybox_back_vertices: Vec<Vertex> = vec![
        [-50.0, -50.0, -50.0],
        [50.0, -50.0, -50.0],
        [50.0, 50.0, -50.0],
        [-50.0, 50.0, -50.0],
    ]
        .iter()
        .map(|v| {
            Vertex {
                position: [v[0], v[1], v[2]],
                tex_coords: [
                    if v[0] == 50.0 { 0.74999 } else { 1.0 },
                    if v[1] == -50.0 { 0.32999 } else { 0.65999 },
                ],
            }
        })
        .collect();


    let ground_normals: Vec<Normal> = vec![[0.0, 1.0, 0.0]; 4]
        .iter()
        .map(|n| {
            Normal {
                normal: [n[0], n[1], n[2]]
            }
        })
        .collect();

    let sky_normals: Vec<Normal> = (0..4)
        .map(|n| {
            Normal {
                normal: [0.0; 3]
            }
        })
        .collect();

    let ground1 = GenericModel::new(&display, &ground1_vertices, &vec![0, 1, 2, 2, 3, 0], &ground_normals);
    let ground2 = GenericModel::new(&display, &ground2_vertices, &vec![0, 1, 2, 2, 3, 0], &ground_normals);
    let road = GenericModel::new(&display, &road_vertices, &vec![0, 1, 2, 2, 3, 0], &ground_normals);
    let skybox_left = GenericModel::new(&display, &skybox_left_vertices, &vec![0, 1, 2, 2, 3, 0], &sky_normals);
    let skybox_front = GenericModel::new(&display, &skybox_front_vertices, &vec![0, 1, 2, 2, 3, 0], &sky_normals);
    let skybox_top = GenericModel::new(&display, &skybox_top_vertices, &vec![0, 1, 2, 2, 3, 0], &sky_normals);
    let skybox_right = GenericModel::new(&display, &skybox_right_vertices, &vec![0, 1, 2, 2, 3, 0], &sky_normals);
    let skybox_back = GenericModel::new(&display, &skybox_back_vertices, &vec![0, 1, 2, 2, 3, 0], &sky_normals);


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
    let railgun1_pos = (-0.21, 0.07, -1.02);
    let railgun2_pos = (-0.41, 0.07, -1.02);

    let fabienne_tex = load_tex!(&display, "../textures/rp_fabienne_percy_posed_001_dif_2k.jpg", jpg);
    let dennis_tex = load_tex!(&display, "../textures/rp_dennis_posed_004_dif_2k.jpg", jpg);
    let altair_tex = load_tex!(&display, "../textures/kaleidoscope.jpg", jpg);
    let ground1_tex = load_tex!(&display, "../textures/grass.jpg", jpg);
    let ground2_tex = load_tex!(&display, "../textures/tough_grass.jpg", jpg);
    let dragon_tex = load_tex!(&display, "../textures/Dragon_ground_color.jpg", jpg);
    let station_tex = load_tex!(&display, "../textures/gasstation red.png", png);
    let bus_tex = load_tex!(&display, "../textures/bus_d.png", png);
    let railgun_tex = load_tex!(&display, "../textures/Railgun_color.jpg", jpg);
    let road_tex = load_tex!(&display, "../textures/road.jpg", jpg);
    let sky_tex = load_tex!(&display, "../textures/dawn.jpg", jpg);

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
            up,
            zfar,
            znear,
            fov,
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
                zfar,
                znear,
                fov,
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
                zfar,
                znear,
                fov,
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
                zfar,
                znear,
                fov,
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
                zfar,
                znear,
                fov,
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
                zfar,
                znear,
                fov,
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
                zfar,
                znear,
                fov,
                ..Default::default()
            }
        );

        railgun1.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotate_self: [spin, railgun_spin_self, 0.],
                scale: 30.17,
                translation: [railgun1_pos.0, railgun1_pos.1, railgun1_pos.2],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                texture: Some(&railgun_tex),
                zfar,
                znear,
                fov,
                ..Default::default()
            }
        );

        railgun2.draw(
            &mut target,
            &draw_params,
            &Transform{
                rotate_self: [spin, railgun_spin_self, 0.],
                scale: 30.17,
                translation: [railgun2_pos.0, railgun2_pos.1, railgun2_pos.2],
                view: [position, direction, up],
                frame_dimensions: Some(dimensions),
                texture: Some(&railgun_tex),
                zfar,
                znear,
                fov,
                ..Default::default()
            }
        );

        ground1.draw(
            &mut target,
            &draw_params,
            &Transform {
                texture: Some(&ground1_tex),
                frame_dimensions: Some(dimensions),
                view: [position, direction, up],
                zfar,
                znear,
                fov,
                ..Default::default()
            }
        );

        ground2.draw(
            &mut target,
            &draw_params,
            &Transform {
                texture: Some(&ground2_tex),
                frame_dimensions: Some(dimensions),
                view: [position, direction, up],
                zfar,
                znear,
                fov,
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
                zfar,
                znear,
                fov,
                ..Default::default()
            }
        );

        skybox_left.draw(
            &mut target,
            &draw_params,
            &Transform {
                texture: Some(&sky_tex),
                frame_dimensions: Some(dimensions),
                view: [position, direction, up],
                zfar,
                znear,
                fov,
                ..Default::default()
            }
        );

        skybox_right.draw(
            &mut target,
            &draw_params,
            &Transform {
                texture: Some(&sky_tex),
                frame_dimensions: Some(dimensions),
                view: [position, direction, up],
                zfar,
                znear,
                fov,
                ..Default::default()
            }
        );

        skybox_top.draw(
            &mut target,
            &draw_params,
            &Transform {
                texture: Some(&sky_tex),
                frame_dimensions: Some(dimensions),
                view: [position, direction, up],
                zfar,
                znear,
                fov,
                ..Default::default()
            }
        );

        skybox_back.draw(
            &mut target,
            &draw_params,
            &Transform {
                texture: Some(&sky_tex),
                frame_dimensions: Some(dimensions),
                view: [position, direction, up],
                zfar,
                znear,
                fov,
                ..Default::default()
            }
        );

        skybox_front.draw(
            &mut target,
            &draw_params,
            &Transform {
                texture: Some(&sky_tex),
                frame_dimensions: Some(dimensions),
                view: [position, direction, up],
                zfar,
                znear,
                fov,
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