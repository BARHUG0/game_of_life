use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: [f32; 2], // [diffuse, specular] reflection coefficients
    pub specular: f32,    // Shininess exponent (higher = smaller, brighter highlight)
                          // Future fields for lighting:
                          // pub reflectivity: f32,
                          // pub transparency: f32,
                          // pub refractive_index: f32,
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
            albedo: [0.9, 0.1], // High diffuse, low specular
            specular: 10.0,     // Soft highlight
        }
    }

    pub fn IVORY() -> Self {
        Material {
            diffuse: Color::new(100, 100, 80, 255),
            albedo: [0.6, 0.3], // Medium diffuse, medium specular
            specular: 50.0,     // Sharp highlight
        }
    }

    pub fn simple(color: Color) -> Self {
        Material {
            diffuse: color,
            albedo: [0.9, 0.5], // Default with some shine
            specular: 32.0,     // Medium shininess
        }
    }
}
