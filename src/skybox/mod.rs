use glium::{Display, DrawParameters, Frame, IndexBuffer, VertexBuffer};
use glium::framebuffer::SimpleFrameBuffer;
use glium::Surface;
use crate::{load_tex, Model, Transform};
use crate::matrices::{view_matrix, perspective_matrix};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

pub struct Skybox {
    cubemap: glium::texture::Cubemap,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    program: glium::Program,
}

impl Model for Skybox {
    fn draw(&self, target: &mut Frame, params: &DrawParameters, transform: &Transform) {
        // Deactivate culling and all for skybox
        let mut params: DrawParameters = params.clone();
        params.depth.test = glium::DepthTest::Overwrite;
        params.depth.write = false;
        params.backface_culling = glium::draw_parameters::BackfaceCullingMode::CullingDisabled;
        params.depth.range = (0.9, 1.0);

        let view = view_matrix(&[1., 0., 0.], &transform.view[1], &[0., 1., 0.]);

        let uniforms = uniform! {
            perspective: perspective_matrix(target),
            view: view,
            cubetex: self.cubemap.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
        };

        target.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniforms, &params).unwrap();
    }
}

impl Skybox {
    pub fn new(display: &Display) -> Self {
        let mut texture = load_tex!(display, "../assets/skybox.png", png);
        let (width, height) = texture.dimensions();
        let mut cubemap = glium::texture::Cubemap::empty(display, width / 4).unwrap();
        Skybox::map_to_cube(display, &mut cubemap, &mut texture, width, height);

        let skybox_vert = include_str!("./skybox.vert");
        let skybox_frag = include_str!("./skybox.frag");

        let program = glium::Program::from_source(
            display,
            skybox_vert,
            skybox_frag,
            None
        ).unwrap();

        Skybox {
            cubemap,
            vertex_buffer: Self::get_vertex_buffer(display),
            index_buffer: Self::get_index_buffer(display),
            program
        }
    }

    fn get_vertex_buffer(display: &Display) -> VertexBuffer<Vertex> {
        let side = 1.;
        let vertices = [
            // left top front
            Vertex { position: [-side, side, side] },
            // right top front
            Vertex { position: [side, side, side] },
            // right bottom front
            Vertex { position: [side, -side, side] },
            // left bottom front
            Vertex { position: [-side, -side, side] },
            // left top back
            Vertex { position: [-side, side, -side] },
            // right top back
            Vertex { position: [side, side, -side] },
            // right bottom back
            Vertex { position: [side, -side, -side] },
            // left bottom back
            Vertex { position: [-side, -side, -side] },
        ];
        VertexBuffer::new(display, &vertices).unwrap()
    }

    fn get_index_buffer(display: &Display) -> IndexBuffer<u16> {
        let indices = [
            // front
            0, 1, 2,
            0, 2, 3,
            // back
            4, 5, 6,
            4, 6, 7,
            // left
            0, 4, 7,
            0, 7, 3,
            // right
            1, 5, 6,
            1, 6, 2,
            // top
            1, 0, 3,
            1, 3, 2,
            // bottom
            4, 7, 6,
            4, 6, 5,
        ];
        glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap()
    }

    fn map_to_cube(
        display: &Display,
        cubemap: &mut glium::texture::Cubemap,
        texture: &mut glium::texture::Texture2d,
        width: u32,
        height: u32)
    {
        let (height_step, width_step) = (height / 3, width / 4);
        let blit_targets = vec![
            // Left
            glium::BlitTarget {
                left: width_step * 0,
                bottom: height_step * 1,
                width: width_step as i32,
                height: height_step as i32,
            },
            // Front
            glium::BlitTarget {
                left: width_step * 1,
                bottom: height_step * 1,
                width: width_step as i32,
                height: height_step as i32,
            },
            // Right
            glium::BlitTarget {
                left: width_step * 2,
                bottom: height_step * 1,
                width: width_step as i32,
                height: height_step as i32,
            },
            // Back
            glium::BlitTarget {
                left: width_step * 3,
                bottom: height_step * 1,
                width: width_step as i32,
                height: height_step as i32,
            },
            // Bottom
            glium::BlitTarget {
                left: width_step * 1,
                bottom: height_step * 0,
                width: width_step as i32,
                height: height_step as i32,
            },
            // Top
            glium::BlitTarget {
                left: width_step * 1,
                bottom: height_step * 2,
                width: width_step as i32,
                height: height_step as i32,
            },
        ];

        let layers = vec![
            // Left
            glium::texture::CubeLayer::NegativeX,
            // Front
            glium::texture::CubeLayer::PositiveZ,
            // Right
            glium::texture::CubeLayer::PositiveX,
            // Back
            glium::texture::CubeLayer::NegativeZ,
            // Bottom
            glium::texture::CubeLayer::NegativeY,
            // Top
            glium::texture::CubeLayer::PositiveY,
        ];

        layers.into_iter()
            .map(|layer| {
                SimpleFrameBuffer::new(
                    display,
                    cubemap.main_level().image(layer),
                ).unwrap()
            })
            .zip(blit_targets)
            .for_each(|(mut fb, target)| {
                texture.as_surface()
                    .blit_whole_color_to(&mut fb, &target, glium::uniforms::MagnifySamplerFilter::Linear);
            });
    }
}