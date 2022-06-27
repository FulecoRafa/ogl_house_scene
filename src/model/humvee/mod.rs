use glium::{DrawParameters, Frame, Surface};
use crate::model::{get_light, Model, ModelData};
use crate::transform::Transform;
use crate::vertex::{Normal, Vertex, Light};
use crate::model::get_program;

mod data;

pub struct Humvee {
    pub model_data: ModelData,
}

impl Model for Humvee {
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
