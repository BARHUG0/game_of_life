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
    pub texture_id: Option<usize>, // NEW: Index into texture pack
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
            texture_id: None, // NEW
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
            texture_id: None, // NEW
        }
    }

    // NEW: Create a material with texture
    pub fn with_texture(mut self, texture_id: usize) -> Self {
        self.texture_id = Some(texture_id);
        self
    }

    // NEW: Check if material uses texture
    pub fn has_texture(&self) -> bool {
        self.texture_id.is_some()
    }
}

impl Material {
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
            texture_id: None,
        }
    }

    /// Glass material with scientifically accurate properties
    /// Refractive index: ~1.5 (typical soda-lime glass)
    /// Transparency: high with minimal surface reflection (~4%)
    /// Source: Standard optical glass properties
    pub fn GLASS(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(245, 250, 255, 255),
            albedo: [0.15, 0.6, 0.1], // CHANGED: Increased diffuse, moderate specular
            specular: 100.0,          // CHANGED: Reduced from 125.0
            reflectivity: 0.08,       // CHANGED: Increased slightly from 0.04
            transparency: 0.85,       // CHANGED: Reduced from 0.95 for visibility
            refractive_index: 1.5,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    /// Diamond material with scientifically accurate properties
    pub fn DIAMOND(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(245, 250, 255, 255),
            albedo: [0.2, 0.9, 0.05], // CHANGED: Increased diffuse component
            specular: 200.0,
            reflectivity: 0.17,
            transparency: 0.00,
            refractive_index: 2.417,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    /// Gold material with scientifically accurate properties
    pub fn GOLD(color: Color, texture_id: Option<usize>) -> Self {
        Material {
            diffuse: color,
            albedo: [0.7, 0.5, 0.0], // CHANGED: Higher diffuse, lower specular
            specular: 80.0,          // CHANGED: Reduced from 150.0
            reflectivity: 0.4,       // CHANGED: Reduced from 0.85
            transparency: 0.0,
            refractive_index: 1.0,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    /// Oak wood material with scientifically accurate properties
    /// Refractive index: ~1.53-1.57 (measured for various wood species)
    /// Diffuse surface with low specular reflection
    /// Source: Wood optical properties research
    pub fn OAK_LOG(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(139, 90, 43, 255),
            albedo: [0.95, 0.05, 0.0], // Highly diffuse, minimal specular
            specular: 8.0,             // Very low specular (rough wood surface)
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.55, // Average wood RI
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    /// Stripped oak log (smoother surface than regular oak)
    pub fn STRIPPED_OAK_LOG(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(176, 133, 84, 255),
            albedo: [0.9, 0.1, 0.0],
            specular: 15.0, // Slightly smoother than regular oak
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.55,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    /// Stone material with scientifically accurate properties
    /// Refractive index: ~1.54-1.55 (quartz/granite composition)
    /// Diffuse surface characteristic of rough stone
    /// Source: Quartz and silicate mineral optical properties
    pub fn STONE(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(125, 125, 125, 255),
            albedo: [0.95, 0.05, 0.0],
            specular: 10.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.545, // Average quartz/stone RI
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    /// Dirt material - highly diffuse, low reflectivity
    pub fn DIRT(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(134, 96, 67, 255),
            albedo: [0.98, 0.02, 0.0], // Almost purely diffuse
            specular: 5.0,             // Very rough surface
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.5, // Approximate for soil particles
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    /// Grass top material - for the top face of grass blocks
    pub fn GRASS_TOP(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(91, 169, 59, 255),
            albedo: [0.95, 0.05, 0.0],
            specular: 8.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.52, // Approximate for plant material
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    /// Leaves material - slightly transparent for realism
    /// Semi-transparent to allow light filtering through
    pub fn LEAVES(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(77, 136, 54, 255),
            albedo: [0.9, 0.1, 0.0],
            specular: 12.0,
            reflectivity: 0.0,
            transparency: 0.15,     // Slight transparency for realistic leaf look
            refractive_index: 1.52, // Plant material RI
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    /// Glowstone material - emissive block
    /// Strong emission for lighting
    pub fn GLOWSTONE(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(233, 197, 120, 255),
            albedo: [0.3, 0.2, 0.0],
            specular: 20.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.5,
            emission: Color::new(255, 220, 140, 255),
            emission_strength: 3.5, // Bright emission
            texture_id,
        }
    }
}
