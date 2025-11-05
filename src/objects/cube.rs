use crate::intersection::Intersect;
use crate::material::Material;
use crate::objects::RayIntersect;
use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Cube {
    pub min: Vector3,
    pub max: Vector3,
    pub materials: [Material; 6], // +X, -X, +Y, -Y, +Z, -Z
}

impl Cube {
    pub fn new(min: Vector3, max: Vector3, materials: [Material; 6]) -> Self {
        Cube {
            min,
            max,
            materials,
        }
    }

    fn get_normal_for_face(face: usize) -> Vector3 {
        match face {
            0 => Vector3::new(1.0, 0.0, 0.0),  // +X
            1 => Vector3::new(-1.0, 0.0, 0.0), // -X
            2 => Vector3::new(0.0, 1.0, 0.0),  // +Y
            3 => Vector3::new(0.0, -1.0, 0.0), // -Y
            4 => Vector3::new(0.0, 0.0, 1.0),  // +Z
            5 => Vector3::new(0.0, 0.0, -1.0), // -Z
            _ => Vector3::new(0.0, 1.0, 0.0),  // fallback
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;
        let mut hit_face: Option<usize> = None;

        // X axis
        {
            let origin = ray_origin.x;
            let direction = ray_direction.x;
            let min_val = self.min.x;
            let max_val = self.max.x;

            if direction.abs() < 1e-6 {
                if origin < min_val || origin > max_val {
                    return Intersect::empty();
                }
            } else {
                let inv_d = 1.0 / direction;
                let mut t1 = (min_val - origin) * inv_d;
                let mut t2 = (max_val - origin) * inv_d;

                let face_in = if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                    1 // -X
                } else {
                    0 // +X
                };

                if t1 > tmin {
                    tmin = t1;
                    hit_face = Some(face_in);
                }
                if t2 < tmax {
                    tmax = t2;
                }
                if tmin > tmax {
                    return Intersect::empty();
                }
            }
        }

        // Y axis
        {
            let origin = ray_origin.y;
            let direction = ray_direction.y;
            let min_val = self.min.y;
            let max_val = self.max.y;

            if direction.abs() < 1e-6 {
                if origin < min_val || origin > max_val {
                    return Intersect::empty();
                }
            } else {
                let inv_d = 1.0 / direction;
                let mut t1 = (min_val - origin) * inv_d;
                let mut t2 = (max_val - origin) * inv_d;

                let face_in = if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                    3 // -Y
                } else {
                    2 // +Y
                };

                if t1 > tmin {
                    tmin = t1;
                    hit_face = Some(face_in);
                }
                if t2 < tmax {
                    tmax = t2;
                }
                if tmin > tmax {
                    return Intersect::empty();
                }
            }
        }

        // Z axis
        {
            let origin = ray_origin.z;
            let direction = ray_direction.z;
            let min_val = self.min.z;
            let max_val = self.max.z;

            if direction.abs() < 1e-6 {
                if origin < min_val || origin > max_val {
                    return Intersect::empty();
                }
            } else {
                let inv_d = 1.0 / direction;
                let mut t1 = (min_val - origin) * inv_d;
                let mut t2 = (max_val - origin) * inv_d;

                let face_in = if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                    5 // -Z
                } else {
                    4 // +Z
                };

                if t1 > tmin {
                    tmin = t1;
                    hit_face = Some(face_in);
                }
                if t2 < tmax {
                    tmax = t2;
                }
                if tmin > tmax {
                    return Intersect::empty();
                }
            }
        }

        if tmin < 0.0 {
            return Intersect::empty();
        }

        let face = hit_face.unwrap_or(0);
        let point = *ray_origin + *ray_direction * tmin;
        let normal = Self::get_normal_for_face(face);

        Intersect::new(self.materials[face], true, tmin, point, normal)
    }
}
