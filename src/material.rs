use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse_color: Color,
    // Future fields for lighting (not implemented yet):
    // pub specular: f32,
    // pub albedo: [f32; 4],
    // pub reflectivity: f32,
    // pub transparency: f32,
    // pub refractive_index: f32,
}

impl Material {
    pub fn new(diffuse_color: Color) -> Self {
        Material { diffuse_color }
    }

    pub fn diffuse_color(&self) -> Color {
        self.diffuse_color
    }

    pub fn default() -> Self {
        Material {
            diffuse_color: Color::new(0, 0, 0, 0),
        }
    }
}

// Preset materials
impl Material {
    pub fn RUBBER() -> Self {
        Material {
            diffuse_color: Color::new(80, 0, 0, 255),
        }
    }

    pub fn IVORY() -> Self {
        Material {
            diffuse_color: Color::new(100, 100, 80, 255),
        }
    }
}
