use glium::{{DrawParameters, Frame, Surface}};
use crate::model::{{Model, ModelData}};
use crate::transform::Transform;

mod data;

pub struct {name} {{
    pub model_data: ModelData,
}}

impl Model for {name} {{
    fn draw(&self, _: &mut Frame, params: &DrawParameters, transform: &Transform) {{
        todo!()
    }}
}}
