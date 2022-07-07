use std::f32::consts::PI;
use glium::glutin::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::ControlFlow;

use crate::{glutin, rotate, translate};
use crate::glutin::event::KeyboardInput;

fn normalize_vector(vector: &[f32; 3]) -> [f32; 3] {
    let length = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
    [
        vector[0] / length,
        vector[1] / length,
        vector[2] / length,
    ]
}

fn cross_product(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn add_vectors(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub_vectors(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vec_scal_mul(vector: &[f32; 3], scalar: f32) -> [f32; 3] {
    [vector[0] * scalar, vector[1] * scalar, vector[2] * scalar]
}

/// Struct that handles the events of the window.
pub struct EventHandler {
    pub grow: f32,
    pub tilt: f32,
    pub spin: f32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub direction: [f32; 3],
    pub position: [f32; 3],
    pub up: [f32; 3],
}

impl EventHandler {
    pub fn new(grow: f32, tilt: f32, spin: f32, translate_x: f32, translate_y: f32, direction: [f32; 3], position: [f32; 3], up: [f32; 3]) -> Self {
        EventHandler {grow, tilt, spin, translate_x, translate_y, direction, position, up}
    }

    /// Method that handles the keyboard input
    pub fn handle_event(&mut self, ev: Event<()>, cf: &mut ControlFlow) {

        let EventHandler {
            ref mut grow,
            ref mut tilt,
            ref mut spin,
            ref mut translate_x,
            ref mut translate_y,
            ref mut direction,
            ref mut position,
            ref mut up,
        } = self;

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    let KeyboardInput {
                        state,
                        virtual_keycode,
                        ..
                    } = input;
                    let virtual_keycode = if let Some(code) = virtual_keycode {
                        code
                    } else {
                        return;
                    };
                    const STEP: f32 = 0.05;
                    /// If the key is pressed, the value is changed
                    if let state = ElementState::Pressed {
                        let camera_facing = normalize_vector(&sub_vectors(&direction, &position));
                        let camera_facing_orth = normalize_vector(&cross_product(&camera_facing, &up));
                        let camera_vert_vec = normalize_vector(&cross_product(&camera_facing_orth, &camera_facing));
                        println!("pos: {:?}, dir: {:?}", position, direction);
                        /// Parses the pressed key and changes the value
                        match virtual_keycode {
                            VirtualKeyCode::W => {
                                *position = add_vectors(&position, &vec_scal_mul(&camera_facing, STEP));
                                *direction = add_vectors(&direction, &vec_scal_mul(&camera_facing, STEP));
                            },
                            VirtualKeyCode::A => {
                                *position = add_vectors(&position, &vec_scal_mul(&camera_facing_orth, STEP));
                                *direction = add_vectors(&direction, &vec_scal_mul(&camera_facing_orth, STEP));
                            }
                            VirtualKeyCode::S => {
                                *position = sub_vectors(&position, &vec_scal_mul(&camera_facing, STEP));
                                *direction = sub_vectors(&direction, &vec_scal_mul(&camera_facing, STEP));
                            }
                            VirtualKeyCode::D => {
                                *position = sub_vectors(&position, &vec_scal_mul(&camera_facing_orth, STEP));
                                *direction = sub_vectors(&direction, &vec_scal_mul(&camera_facing_orth, STEP));
                            }
                            VirtualKeyCode::J => *spin += STEP,
                            VirtualKeyCode::K => *spin -= STEP,
                            VirtualKeyCode::Right => {
                                *direction = sub_vectors(&direction, &camera_facing_orth);
                            }
                            VirtualKeyCode::Left => {
                                *direction = add_vectors(&direction, &camera_facing_orth);
                            }
                            VirtualKeyCode::Up => {
                                *direction = add_vectors(&direction, &camera_vert_vec);
                            }
                            VirtualKeyCode::Down => {
                                *direction = sub_vectors(&direction, &camera_vert_vec);
                            }
                            _ => (),
                        }
                    }

                    if *grow < 0.02 {
                        *grow = 0.01;
                    } else if *grow > 1.0 {
                        *grow = 1.0;
                    }

                    if *tilt > 1.0 {
                        *tilt = 1.0;
                    } else if *tilt < -1.0 {
                        *tilt = -1.0;
                    }

                }

                glutin::event::WindowEvent::CloseRequested => {
                    *cf = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        EventHandler {
            grow: 1.0,
            tilt: 0.0,
            spin: 0.0,
            translate_x: 0.0,
            translate_y: 0.0,
            direction: [0.0, 0.0, 1.0],
            position: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
        }
    }
}
