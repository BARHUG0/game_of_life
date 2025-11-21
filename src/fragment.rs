use raylib::prelude::*;

#[derive(Clone, Debug)]
pub struct Fragment {
    position: Vector2,
    world_position: Vector3,
    object_position: Vector3,
    normal: Vector3,
    depth: f32,
    tex_coords: Vector2,
}

impl Fragment {
    pub fn new(
        position: Vector2,
        world_position: Vector3,
        object_position: Vector3,
        normal: Vector3,
        depth: f32,
        tex_coords: Vector2,
    ) -> Self {
        Fragment {
            position,
            world_position,
            object_position,
            normal,
            depth,
            tex_coords,
        }
    }

    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn world_position(&self) -> Vector3 {
        self.world_position
    }

    pub fn object_position(&self) -> Vector3 {
        self.object_position
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    pub fn depth(&self) -> f32 {
        self.depth
    }

    pub fn tex_coords(&self) -> Vector2 {
        self.tex_coords
    }
}
