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

    pub fn colored(position: Vector3, intensity: f32, color: Color) -> Self {
        Light {
            position,
            intensity,
            color,
        }
    }

    // Calculate light intensity with distance attenuation
    pub fn get_intensity_at(&self, point: &Vector3) -> f32 {
        let distance = (self.position - *point).length();
        // Attenuation formula: intensity / (1 + distance^2)
        // This prevents division by zero and gives smooth falloff
        self.intensity / (1.0 + distance * distance * 0.1)
    }
}
