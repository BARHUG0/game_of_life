mod cube;
mod sphere;

pub use cube::Cube;
pub use sphere::Sphere;

use crate::intersection::Intersect;
use raylib::prelude::*;

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect;
}

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
    Cube(Cube),
}

impl RayIntersect for Object {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        match self {
            Object::Sphere(sphere) => sphere.ray_intersect(ray_origin, ray_direction),
            Object::Cube(cube) => cube.ray_intersect(ray_origin, ray_direction),
        }
    }
}
