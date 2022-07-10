use glium::{Display, DrawParameters, Frame, IndexBuffer, Surface, VertexBuffer};

use crate::assets::{
    transform::*,
    vertex::*,
};
use crate::model::{get_light, get_light_rotation_matrix, Model, ModelData};
use crate::model::get_program;
use crate::model::model_parser::parse_model;

pub struct GenericModel {
    pub model_data: ModelData,
}

impl GenericModel {

    /// Creates a new GenericModel from given the indices, normals and vertices
    pub fn new(display: &Display, vertices: &Vec<Vertex>, indices: &Vec<u32>, normals: &Vec<Normal>) -> GenericModel {
        let model_data = ModelData {
            vertices: VertexBuffer::new(display, vertices).unwrap(),
            indices: IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, indices).unwrap(),
            normals: VertexBuffer::new(display, normals).unwrap(),
        };
        GenericModel {
            model_data,
        }
    }

    /// Creates a new GenericModel from given the path to the model file
    pub fn from_obj(display: &Display, obj_src: String) -> Self {

        let (vertices, indices, normals) = parse_model(&obj_src);

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
    /// Draws the model
    fn draw(&self, target: &mut Frame, params: &DrawParameters, transform: &Transform) {
        let uniforms = uniform! {
            translation: transform.get_translation(),
            scale: transform.get_scaling(),
            rotation: transform.get_rotation(),
            self_rotation: transform.get_self_rotation(),
            view: transform.get_view(),
            perspective: transform.get_perspective(),
            light: get_light(),
            light_rotation: get_light_rotation_matrix(),
            tex: transform.get_texture(),
        };
        target.draw((&self.model_data.vertices, &self.model_data.normals), &self.model_data.indices, get_program().unwrap(), &uniforms, params).unwrap();
    }
}


