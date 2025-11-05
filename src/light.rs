use raylib::prelude::*;

// Placeholder for future lighting implementation
#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vector3,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vector3, intensity: f32) -> Self {
        Light {
            position,
            intensity,
        }
    }
}
