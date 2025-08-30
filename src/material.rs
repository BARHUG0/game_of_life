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
