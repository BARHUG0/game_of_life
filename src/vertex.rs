use raylib::prelude::*;

#[derive(Clone, Debug)]
pub struct Vertex {
    position: Vector3,
    normal: Vector3,
    tex_coords: Vector2,
}

impl Vertex {
    pub fn new(position: Vector3, normal: Vector3, tex_coords: Vector2) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
        }
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    pub fn tex_coords(&self) -> Vector2 {
        self.tex_coords
    }
}
