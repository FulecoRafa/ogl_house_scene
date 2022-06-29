use glium::{Display, DrawParameters, Frame, Surface, VertexBuffer, IndexBuffer};
use crate::model::{get_light, Model, ModelData};
use crate::transform::Transform;
use crate::vertex::{Normal, Vertex, Light};
use crate::model::get_program;
use crate::model::model_parser::parse_model;

pub struct GenericModel {
    pub model_data: ModelData,
}

impl GenericModel {
    pub fn new(display: &Display, model_src: String) -> Self {

        let (vertices, indices, normals) = parse_model(&model_src);

        GenericModel {
            model_data: ModelData {
                vertices: VertexBuffer::new(display, &vertices[..]).unwrap(),
                indices: IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices[..]).unwrap(),
                normals: VertexBuffer::new(display, &normals[..]).unwrap(),
            }
        }
    }
}

impl Model for GenericModel {
    fn draw(&self, target: &mut Frame, params: &DrawParameters, transform: &Transform) {
        let uniforms = uniform! {
            translation: transform.get_translation(),
            scale: transform.get_scaling(),
            rotation: transform.get_rotation(),
            self_rotation: transform.get_self_rotation(),
            view: transform.get_view(),
            light: get_light(),
        };
        target.draw((&self.model_data.vertices, &self.model_data.normals), &self.model_data.indices, get_program().unwrap(), &uniforms, params).unwrap();
    }
}

