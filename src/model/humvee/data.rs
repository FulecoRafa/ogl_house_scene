use glium::{Display, IndexBuffer, VertexBuffer};
use crate::vertex::{Vertex, Normal};
use crate::model::{ModelData, humvee::Humvee};
use crate::model_parser::parse_model;

impl Humvee {
    pub fn new(display:  &Display) -> Self {

        let (vertices, indices, normals) = parse_model("models/Humvee.obj");

        println!("{:?}", vertices[0].position);

        Humvee {
            model_data: ModelData {
                vertices: VertexBuffer::new(display, &vertices[..]).unwrap(),
                indices: IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices[..]).unwrap(),
                normals: VertexBuffer::new(display, &normals[..]).unwrap(),
            }
        }
    }
}
