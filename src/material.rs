use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: [f32; 3], // [diffuse, specular, refraction]
    pub specular: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub emission: Color,
    pub emission_strength: f32,
}

impl Material {
    pub fn new(
        diffuse: Color,
        albedo: [f32; 3],
        specular: f32,
        reflectivity: f32,
        transparency: f32,
        refractive_index: f32,
        emission: Color,
        emission_strength: f32,
    ) -> Self {
        Material {
            diffuse,
            albedo,
            specular,
            reflectivity,
            transparency,
            refractive_index,
            emission,
            emission_strength,
        }
    }

    pub fn diffuse_color(&self) -> Color {
        self.diffuse
    }

    pub fn default() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0, 0),
            albedo: [1.0, 0.0, 0.0],
            specular: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
        }
    }
}

// Preset materials
impl Material {
    pub fn RUBBER() -> Self {
        Material {
            diffuse: Color::new(80, 0, 0, 255),
            albedo: [0.9, 0.1, 0.0],
            specular: 10.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
        }
    }

    pub fn IVORY() -> Self {
        Material {
            diffuse: Color::new(100, 100, 80, 255),
            albedo: [0.6, 0.3, 0.1],
            specular: 50.0,
            reflectivity: 0.2,
            transparency: 0.0,
            refractive_index: 1.0,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
        }
    }

    pub fn simple(color: Color) -> Self {
        Material {
            diffuse: color,
            albedo: [0.9, 0.3, 0.0],
            specular: 50.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
        }
    }

    pub fn MIRROR() -> Self {
        Material {
            diffuse: Color::new(255, 255, 255, 255),
            albedo: [0.0, 0.8, 0.1],
            specular: 100.0,
            reflectivity: 0.9,
            transparency: 0.0,
            refractive_index: 1.0,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
        }
    }

    pub fn METAL(color: Color) -> Self {
        Material {
            diffuse: color,
            albedo: [0.6, 0.8, 0.0],
            specular: 80.0,
            reflectivity: 0.5,
            transparency: 0.0,
            refractive_index: 1.0,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
        }
    }

    // Updated GLASS with proper refraction
    pub fn GLASS() -> Self {
        Material {
            diffuse: Color::new(200, 220, 255, 255),
            albedo: [0.0, 0.5, 0.1],
            specular: 125.0,
            reflectivity: 0.1,
            transparency: 0.9,
            refractive_index: 1.5,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
        }
    }

    // Additional presets
    pub fn WATER() -> Self {
        Material {
            diffuse: Color::new(100, 150, 255, 255),
            albedo: [0.0, 0.6, 0.1],
            specular: 100.0,
            reflectivity: 0.1,
            transparency: 0.8,
            refractive_index: 1.33,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
        }
    }

    pub fn DIAMOND() -> Self {
        Material {
            diffuse: Color::new(240, 240, 255, 255),
            albedo: [0.0, 0.9, 0.05],
            specular: 200.0,
            reflectivity: 0.2,
            transparency: 0.9,
            refractive_index: 2.42,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
        }
    }

    // === EMISSIVE MATERIALS ===

    pub fn EMISSIVE(color: Color, strength: f32) -> Self {
        Material {
            diffuse: color,
            albedo: [0.2, 0.1, 0.0],
            specular: 10.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            emission: color,
            emission_strength: strength,
        }
    }

    pub fn LAVA() -> Self {
        Material {
            diffuse: Color::new(255, 100, 0, 255),
            albedo: [0.3, 0.1, 0.0],
            specular: 20.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            emission: Color::new(255, 80, 0, 255),
            emission_strength: 2.5,
        }
    }

    pub fn LED(color: Color) -> Self {
        Material {
            diffuse: color,
            albedo: [0.1, 0.3, 0.0],
            specular: 80.0,
            reflectivity: 0.1,
            transparency: 0.0,
            refractive_index: 1.0,
            emission: color,
            emission_strength: 4.0,
        }
    }

    pub fn GLOW_GLASS(color: Color) -> Self {
        Material {
            diffuse: color,
            albedo: [0.0, 0.4, 0.1],
            specular: 125.0,
            reflectivity: 0.1,
            transparency: 0.8,
            refractive_index: 1.5,
            emission: color,
            emission_strength: 1.5,
        }
    }
}
