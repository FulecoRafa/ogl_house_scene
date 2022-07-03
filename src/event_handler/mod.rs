use std::f32::consts::PI;
use glium::glutin::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::ControlFlow;

use crate::glutin;
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
                    const ANGLE_STEP: f32 = 0.0005;
                    /// If the key is pressed, the value is changed
                    if let state = ElementState::Pressed {
                        println!("pos: {:?}, dir: {:?}, atanyz: {}, atanxz :{}", position, direction, (direction[1] - position[1]).atan2(direction[2] - position[2]), (direction[0] - position[0]).atan2(direction[2] - position[2]));
                        let norm_cross = normalize_vector(&cross_product(direction, up));
                        let norm_direction = normalize_vector(direction);
                        /// Parses the pressed key and changes the value
                        match virtual_keycode {
                            VirtualKeyCode::W => {
                                *position = [position[0] + norm_direction[0] * STEP, position[1] + norm_direction[1] * STEP, position[2] + norm_direction[2] * STEP];
                                *direction = [direction[0] + norm_direction[0] * STEP, direction[1] + norm_direction[1] * STEP, direction[2] + norm_direction[2] * STEP];
                            },
                            VirtualKeyCode::A => {
                                *position = [position[0] + norm_cross[0] * STEP, position[1] + norm_cross[1] * STEP, position[2] + norm_cross[2] * STEP];
                                *direction = [direction[0] + norm_cross[0] * STEP, direction[1] + norm_cross[1] * STEP, direction[2] + norm_cross[2] * STEP];
                            }
                            VirtualKeyCode::S => {
                                *position = [position[0] - norm_direction[0] * STEP, position[1] - norm_direction[1] * STEP, position[2] - norm_direction[2] * STEP];
                                *direction = [direction[0] - norm_direction[0] * STEP, direction[1] - norm_direction[1] * STEP, direction[2] - norm_direction[2] * STEP];
                            }
                            VirtualKeyCode::D => {
                                *position = [position[0] - norm_cross[0] * STEP, position[1] - norm_cross[1] * STEP, position[2] - norm_cross[2] * STEP];
                                *direction = [direction[0] - norm_cross[0] * STEP, direction[1] - norm_cross[1] * STEP, direction[2] - norm_cross[2] * STEP];
                            }
                            VirtualKeyCode::J => *spin += STEP,
                            VirtualKeyCode::K => *spin -= STEP,
                            VirtualKeyCode::Right => {
                                /// Translate direction to position
                                let x = direction[0] - position[0];
                                let y = direction[1] - position[1];
                                let z = direction[2] - position[2];

                                let mut theta = (x.atan2(z) + 2. * PI) % (2. * PI);
                                let len = (x * x + y * y + z * z).sqrt();
                                theta += STEP;

                                *direction = [
                                    theta.sin() * len + position[0],
                                    direction[1],
                                    theta.cos() * len + position[2],
                                ];
                            }
                            VirtualKeyCode::Left => {
                                let x = direction[0] - position[0];
                                let y = direction[1] - position[1];
                                let z = direction[2] - position[2];

                                let mut theta = (x.atan2(z) + 2. * PI) % (2. * PI);
                                let len = (x * x + y * y + z * z).sqrt();
                                theta -= STEP;

                                *direction = [
                                    theta.sin() * len + position[0],
                                    direction[1],
                                    theta.cos() * len + position[2],
                                ];
                            }
                            VirtualKeyCode::Up => {
                                let x = direction[0] - position[0];
                                let y = direction[1] - position[1];
                                let z = direction[2] - position[2];

                                let mut theta = (y.atan2(z) + 2. * PI) % (2. * PI);
                                let len = (y * y + z * z).sqrt();
                                theta += STEP;

                                *direction = [
                                    direction[0],
                                    theta.sin() * len + position[1],
                                    theta.cos() * len + position[2],
                                ];
                            }
                            VirtualKeyCode::Down => {
                                let x = direction[0] - position[0];
                                let y = direction[1] - position[1];
                                let z = direction[2] - position[2];

                                let mut theta = (y.atan2(z) + 2. * PI) % (2. * PI);
                                let len = (y * y + z * z).sqrt();
                                theta -= STEP;

                                *direction = [
                                    direction[0],
                                    theta.sin() * len + position[1],
                                    theta.cos() * len + position[2],
                                ];
                            }
                            VirtualKeyCode::Numpad8 => (*position)[1] -= STEP,
                            VirtualKeyCode::Numpad2 => (*position)[1] += STEP,
                            VirtualKeyCode::Numpad4 => (*position)[0] += STEP,
                            VirtualKeyCode::Numpad6 => (*position)[0] -= STEP,
                            VirtualKeyCode::Numpad5 => (*position)[2] += STEP,
                            VirtualKeyCode::NumpadSubtract => *translate_x -= STEP,
                            VirtualKeyCode::NumpadAdd => *translate_x += STEP,
                            VirtualKeyCode::F1 => (*direction)[0] -= STEP,
                            VirtualKeyCode::F2 => (*direction)[0] += STEP,
                            VirtualKeyCode::F3 => (*direction)[1] -= STEP,
                            VirtualKeyCode::F4 => (*direction)[1] += STEP,
                            VirtualKeyCode::F5 => (*direction)[2] += STEP,
                            VirtualKeyCode::F6 => (*direction)[2] -= STEP,
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
