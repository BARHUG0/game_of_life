use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vector3,
    pub intensity: f32,
    pub color: Color,
}

impl Light {
    pub fn new(position: Vector3, intensity: f32, color: Color) -> Self {
        Light {
            position,
            intensity,
            color,
        }
    }

    pub fn white(position: Vector3, intensity: f32) -> Self {
        Light {
            position,
            intensity,
            color: Color::WHITE,
        }
    }
}
