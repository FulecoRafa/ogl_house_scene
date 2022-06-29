#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: [f32; 3],
}

implement_vertex!(Normal, normal);

pub type Light = [f32; 3];