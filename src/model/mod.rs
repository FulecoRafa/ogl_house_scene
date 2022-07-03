use glium::{Display, IndexBuffer, Program, VertexBuffer};

use crate::assets::{
    matrices::*,
    transform::*,
    vertex::*,
};
use crate::rotate;

pub mod generic_model;
mod model_parser;

const VERT_SHADER: &str = include_str!("shaders/shader.vert");
const FRAG_SHADER: &str = include_str!("shaders/shader.frag");

static mut PROGRAM: Option<Program> = None;
pub fn set_program(display: &Display) {
    unsafe {
        if let None = PROGRAM {
            PROGRAM = Some(
                Program::from_source(
                    display,
                    VERT_SHADER,
                    FRAG_SHADER,
                    None
                ).unwrap(),
            );
        }
    }
}

pub fn get_program() -> Option<&'static Program> {
    unsafe {
        PROGRAM.as_ref()
    }
}

static mut LIGHT: Light = [1.0, 1.0, 1.0f32];
static mut LIGHT_ROTATION: f32 = 0.0;

pub fn set_light_rotation(light_rotation: f32) {
    unsafe {
        LIGHT_ROTATION = light_rotation;
    }
}

pub fn get_light_rotation_matrix() -> [[f32; 4]; 4] {
    unsafe{
        rotate!(LIGHT_ROTATION.clone(), x)
    }
}

pub fn get_light_rotation() -> f32 {
    unsafe {
        LIGHT_ROTATION.clone()
    }
}

pub fn set_light(light: Light) {
    unsafe {
        LIGHT = light;
    }
}

pub fn get_light() -> Light {
    unsafe {
        LIGHT.clone()
    }
}

pub struct ModelData {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u32>,
    pub normals: VertexBuffer<Normal>,
}

pub trait Model {
    fn draw(&self, _: &mut glium::Frame, params: &glium::DrawParameters, transform: &Transform);
}
