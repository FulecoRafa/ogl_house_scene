use crate::{rotate, scale, translate};
use crate::matrices::view_matrix;

/// Struct that holds the transform parameters of a drawable object.
#[derive(Clone)]
pub struct Transform {
    /// Translate in [x, y, z]
    pub translation: [f32; 3],
    /// Rotate in [x, y, z]
    pub rotation: [f32; 3],
    /// Rotate object around itself in [x, y, z]
    pub rotate_self: [f32; 3],
    /// Scale in s
    pub scale: f32,
    /// View in [direction, position, up]
    pub view: [[f32; 3]; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            rotate_self: [0.0, 0.0, 0.0],
            scale: 0.25,
            view: [[1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        }
    }
}

impl Transform {
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
}