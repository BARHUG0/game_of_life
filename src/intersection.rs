use crate::material::Material;
use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub material: Material,
    pub is_intersecting: bool,
    pub distance: f32,
    pub point: Vector3,
    pub normal: Vector3,
    pub uv: Vector2,       // NEW: UV coordinates for texture sampling
    pub face_index: usize, // NEW: Track which face was hit (for cubes)
}

impl Intersect {
    pub fn new(
        material: Material,
        is_intersecting: bool,
        distance: f32,
        point: Vector3,
        normal: Vector3,
        uv: Vector2,       // NEW: Added UV parameter
        face_index: usize, // NEW: Track which face was hit (for cubes)
    ) -> Self {
        Intersect {
            is_intersecting,
            material,
            distance,
            point,
            normal,
            uv, // NEW
            face_index,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            material: Material::default(),
            is_intersecting: false,
            distance: 0.0,
            point: Vector3::zero(),
            normal: Vector3::zero(),
            uv: Vector2::zero(), // NEW
            face_index: 0,
        }
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn is_intersecting(&self) -> bool {
        self.is_intersecting
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }
}
