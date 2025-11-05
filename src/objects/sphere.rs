use crate::intersection::Intersect;
use crate::material::Material;
use crate::objects::RayIntersect;
use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Vector3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        let oc = *ray_origin - self.center;
        let a = ray_direction.dot(*ray_direction);
        let b = 2.0 * oc.dot(*ray_direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersect::empty();
        }

        let sqrt_disc = discriminant.sqrt();
        let t1 = (-b - sqrt_disc) / (2.0 * a);
        let t2 = (-b + sqrt_disc) / (2.0 * a);

        let distance = if t1 > 0.0 {
            t1
        } else if t2 > 0.0 {
            t2
        } else {
            return Intersect::empty();
        };

        let point = *ray_origin + *ray_direction * distance;
        let normal = (point - self.center).normalized();

        Intersect::new(self.material, true, distance, point, normal)
    }
}
