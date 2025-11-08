use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vector3,
    pub intensity: f32,
    pub color: Color,
    pub radius: f32, // NEW: For soft shadows
}

impl Light {
    pub fn new(position: Vector3, intensity: f32, color: Color) -> Self {
        Light {
            position,
            intensity,
            color,
            radius: 0.0, // Default to hard shadows
        }
    }

    // NEW: Constructor with radius for soft shadows
    pub fn soft(position: Vector3, intensity: f32, color: Color, radius: f32) -> Self {
        Light {
            position,
            intensity,
            color,
            radius,
        }
    }

    pub fn white(position: Vector3, intensity: f32) -> Self {
        Light {
            position,
            intensity,
            color: Color::WHITE,
            radius: 0.0,
        }
    }

    pub fn colored(position: Vector3, intensity: f32, color: Color) -> Self {
        Light {
            position,
            intensity,
            color,
            radius: 0.0,
        }
    }

    pub fn get_intensity_at(&self, point: &Vector3) -> f32 {
        let distance = (self.position - *point).length();
        self.intensity / (1.0 + distance * distance * 0.1)
    }
}
