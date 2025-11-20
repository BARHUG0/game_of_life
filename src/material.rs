use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: [f32; 3],
    pub specular: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub emission: Color,
    pub emission_strength: f32,
    pub texture_id: Option<usize>,
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
            texture_id: None,
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
            texture_id: None,
        }
    }

    pub fn with_texture(mut self, texture_id: usize) -> Self {
        self.texture_id = Some(texture_id);
        self
    }

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

    pub fn GLASS(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(245, 250, 255, 255),
            albedo: [0.2, 0.6, 0.1],
            specular: 100.0,
            reflectivity: 0.08,
            transparency: 0.95,
            refractive_index: 1.5,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn DIAMOND(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(245, 250, 255, 255),
            albedo: [0.2, 0.9, 0.05],
            specular: 200.0,
            reflectivity: 0.17,
            transparency: 0.00,
            refractive_index: 2.417,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn GOLD(color: Color, texture_id: Option<usize>) -> Self {
        Material {
            diffuse: color,
            albedo: [0.7, 0.5, 0.0],
            specular: 80.0,
            reflectivity: 0.4,
            transparency: 0.0,
            refractive_index: 1.0,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn OAK_LOG(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(139, 90, 43, 255),
            albedo: [0.95, 0.05, 0.0],
            specular: 8.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.55,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn STRIPPED_OAK_LOG(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(176, 133, 84, 255),
            albedo: [0.9, 0.1, 0.0],
            specular: 15.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.55,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn STONE(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(125, 125, 125, 255),
            albedo: [0.95, 0.05, 0.0],
            specular: 10.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.545,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn DIRT(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(134, 96, 67, 255),
            albedo: [0.98, 0.02, 0.0],
            specular: 5.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.5,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn GRASS_TOP(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(91, 169, 59, 255),
            albedo: [0.95, 0.05, 0.0],
            specular: 8.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.52,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn LEAVES(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(77, 136, 54, 255),
            albedo: [0.9, 0.1, 0.0],
            specular: 12.0,
            reflectivity: 0.0,
            transparency: 0.55,
            refractive_index: 1.52,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn GLOWSTONE(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(233, 197, 120, 255),
            albedo: [0.3, 0.2, 0.0],
            specular: 20.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.5,
            emission: Color::new(255, 220, 140, 255),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn EMERALD_ORE(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(115, 125, 115, 255),
            albedo: [0.90, 0.10, 0.0],
            specular: 25.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.545,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn IRON_ORE(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(135, 130, 126, 255),
            albedo: [0.95, 0.05, 0.0],
            specular: 20.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.545,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn HONEYCOMB(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(229, 148, 58, 255),
            albedo: [0.92, 0.08, 0.0],
            specular: 18.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.47,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn ICE(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(145, 180, 240, 255),
            albedo: [0.25, 0.75, 0.0],
            specular: 90.0,
            reflectivity: 0.15,
            transparency: 0.0,
            refractive_index: 1.31,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn TNT(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(219, 71, 54, 255),
            albedo: [0.98, 0.02, 0.0],
            specular: 5.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.5,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }

    pub fn STRIPPED_OAK_LOG_NEW(texture_id: Option<usize>) -> Self {
        Material {
            diffuse: Color::new(176, 133, 84, 255),
            albedo: [0.88, 0.12, 0.0],
            specular: 20.0,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.55,
            emission: Color::new(0, 0, 0, 0),
            emission_strength: 0.0,
            texture_id,
        }
    }
}
