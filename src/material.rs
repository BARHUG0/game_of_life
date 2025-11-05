use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: f32, // Diffuse reflection coefficient [0.0 - 1.0]
                     // Future fields for lighting:
                     // pub specular: f32,
                     // pub reflectivity: f32,
                     // pub transparency: f32,
                     // pub refractive_index: f32,
}

impl Material {
    pub fn new(diffuse: Color, albedo: f32) -> Self {
        Material { diffuse, albedo }
    }

    pub fn diffuse_color(&self) -> Color {
        self.diffuse
    }

    pub fn default() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0, 0),
            albedo: 1.0,
        }
    }
}

// Preset materials
impl Material {
    pub fn RUBBER() -> Self {
        Material {
            diffuse: Color::new(80, 0, 0, 255),
            albedo: 0.9,
        }
    }

    pub fn IVORY() -> Self {
        Material {
            diffuse: Color::new(100, 100, 80, 255),
            albedo: 0.6,
        }
    }

    pub fn simple(color: Color) -> Self {
        Material {
            diffuse: color,
            albedo: 1.0,
        }
    }
}
