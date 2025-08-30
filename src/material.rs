use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse_color: Color,
}

impl Material {
    pub fn new(diffuse_color: Color) -> Self {
        Material { diffuse_color }
    }

    pub fn diffuse_color(&self) -> Color {
        self.diffuse_color
    }
}

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
