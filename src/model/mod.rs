use glium::{Display, IndexBuffer, Program, VertexBuffer};
use crate::transform::Transform;
use crate::vertex::{Normal, Vertex, Light};

pub mod humvee;

const VERT_SHADER: &str = include_str!("./shader.vert");
const FRAG_SHADER: &str = include_str!("./shader.frag");

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

static mut LIGHT: Light = [0.0, 0.0, 0.0];

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