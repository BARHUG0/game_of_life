use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: [f32; 2], // [diffuse, specular] reflection coefficients
    pub specular: f32,    // Shininess exponent (higher = smaller, brighter highlight)
}

impl Material {
    pub fn new(diffuse: Color, albedo: [f32; 2], specular: f32) -> Self {
        Material {
            diffuse,
            albedo,
            specular,
        }
    }

    pub fn diffuse_color(&self) -> Color {
        self.diffuse
    }

    pub fn default() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0, 0),
            albedo: [1.0, 0.0],
            specular: 0.0,
        }
    }
}

// Preset materials
impl Material {
    pub fn RUBBER() -> Self {
        Material {
            diffuse: Color::new(80, 0, 0, 255),
            albedo: [0.9, 0.1],
            specular: 10.0,
        }
    }

    pub fn IVORY() -> Self {
        Material {
            diffuse: Color::new(100, 100, 80, 255),
            albedo: [0.6, 0.3],
            specular: 50.0,
        }
    }

    // ADJUSTED: Reduced specular albedo and increased shininess
    // This prevents over-bright specular highlights while keeping nice reflections
    pub fn simple(color: Color) -> Self {
        Material {
            diffuse: color,
            albedo: [0.9, 0.3], // Reduced from 0.5 to 0.3
            specular: 50.0,     // Increased from 32.0 to 50.0 (tighter highlight)
        }
    }
}
