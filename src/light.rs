use raylib::prelude::*;

pub struct Light {
    position: Vector3,
}

impl Light {
    pub fn new(position: Vector3) -> Self {
        Light { position }
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }
}
