use glium::{Frame, texture::SrgbTexture2d};
use crate::{identity, rotate, scale, translate};
use crate::assets::matrices::{perspective_matrix, view_matrix};

/// Struct that holds the transform parameters of a drawable object.
pub struct Transform<'a> {
    /// Translate in [x, y, z]
    pub translation: [f32; 3],
    /// Rotate in [x, y, z]
    pub rotation: [f32; 3],
    /// Rotate object around itself in [x, y, z]
    pub rotate_self: [f32; 3],
    /// Scale in s
    pub scale: f32,
    /// View in [position, direction, up]
    pub view: [[f32; 3]; 3],
    /// Frame
    pub frame_dimensions: Option<(u32, u32)>,
    // Texture
    pub texture: Option<&'a glium::texture::SrgbTexture2d>,
}

impl Default for Transform<'_> {
    fn default() -> Self {
        Transform {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            rotate_self: [0.0, 0.0, 0.0],
            scale: 0.25,
            view: [[1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            frame_dimensions: None,
            texture: None
        }
    }
}

impl Transform<'_> {
    pub fn get_translation(&self) -> [[f32; 4]; 4] {
        translate!(self.translation[0], self.translation[1], self.translation[2])
    }

    pub fn get_rotation(&self) -> [[f32; 4]; 4] {
        rotate!(self.rotation[0], self.rotation[1], self.rotation[2])
    }

    pub fn get_scaling(&self) -> [[f32; 4]; 4] {
        scale!(self.scale)
    }

    pub fn get_self_rotation(&self) -> [[f32; 4]; 4] {
        rotate!(self.rotate_self[0], self.rotate_self[1], self.rotate_self[2])
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        view_matrix(&self.view[0], &self.view[1], &self.view[2])
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        match self.frame_dimensions {
            Some(dim) => perspective_matrix(dim),
            None => identity!(),
        }
    }

    pub fn get_texture(&self) -> &SrgbTexture2d {
        self.texture.unwrap()
    }
}