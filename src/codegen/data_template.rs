use glium::{{Display, IndexBuffer, VertexBuffer}};
use crate::vertex::{{Vertex, Normal}};
use crate::model::{{ModelData, {name_lowercase}::{name}}};


impl {name} {{
    pub fn new(display:  &Display) -> Self {{
        let vertices: [Vertex; {vertices_len}] = [{vertices}];
        let indices: [u32; {indices_len}] = [{indices}];
        let normals: [Normal; {normals_len}] = [{normals}];
        {name} {{
            model_data: ModelData {{
                vertices: VertexBuffer::new(display, &vertices[..]).unwrap(),
                indices: IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices[..]).unwrap(),
                normals: VertexBuffer::new(display, &normals[..]).unwrap(),
            }}
        }}
    }}
}}
