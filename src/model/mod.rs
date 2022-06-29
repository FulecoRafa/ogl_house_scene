use glium::{Display, IndexBuffer, Program, VertexBuffer};

use crate::assets::{
    matrices::*,
    transform::*,
    vertex::*,
};

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

static mut LIGHT: Light = [-1.0, 0.4, 0.9f32];

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
