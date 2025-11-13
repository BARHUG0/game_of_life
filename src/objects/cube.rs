use crate::intersection::Intersect;
use crate::material::Material;
use crate::objects::RayIntersect;
use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Cube {
    pub min: Vector3,
    pub max: Vector3,
    pub center: Vector3,
    pub materials: [Material; 6], // +X, -X, +Y, -Y, +Z, -Z
}

impl Cube {
    pub fn new(min: Vector3, max: Vector3, center: Vector3, materials: [Material; 6]) -> Self {
        Cube {
            min,
            max,
            center,
            materials,
        }
    }

    pub fn new_with_center(center: Vector3, size: f32, materials: [Material; 6]) -> Self {
        let half = size * 0.5;
        Cube {
            min: center - Vector3::new(half, half, half),
            max: center + Vector3::new(half, half, half),
            center,
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
            _ => Vector3::new(0.0, 1.0, 0.0),
        }
    }

    // NEW: Calculate UV coordinates for a face based on hit point
    fn calculate_uv(&self, point: &Vector3, face: usize) -> Vector2 {
        // Normalize point to [0, 1] range within the cube
        let local_x = (point.x - self.min.x) / (self.max.x - self.min.x);
        let local_y = (point.y - self.min.y) / (self.max.y - self.min.y);
        let local_z = (point.z - self.min.z) / (self.max.z - self.min.z);

        match face {
            0 => Vector2::new(1.0 - local_z, 1.0 - local_y), // +X face (looking at -X)
            1 => Vector2::new(local_z, 1.0 - local_y),       // -X face (looking at +X)
            2 => Vector2::new(local_x, 1.0 - local_z),       // +Y face (looking down)
            3 => Vector2::new(local_x, local_z),             // -Y face (looking up)
            4 => Vector2::new(local_x, 1.0 - local_y),       // +Z face (looking at -Z)
            5 => Vector2::new(1.0 - local_x, 1.0 - local_y), // -Z face (looking at +Z)
            _ => Vector2::new(0.0, 0.0),
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;
        let mut hit_face: usize = 0;

        let origins = [ray_origin.x, ray_origin.y, ray_origin.z];
        let directions = [ray_direction.x, ray_direction.y, ray_direction.z];
        let mins = [self.min.x, self.min.y, self.min.z];
        let maxs = [self.max.x, self.max.y, self.max.z];

        for axis in 0..3 {
            let origin = origins[axis];
            let direction = directions[axis];
            let min_val = mins[axis];
            let max_val = maxs[axis];

            if direction.abs() > 1e-6 {
                let inv_d = 1.0 / direction;
                let mut t1 = (min_val - origin) * inv_d;
                let mut t2 = (max_val - origin) * inv_d;

                let face = if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                    axis * 2 + 1 // Negative face
                } else {
                    axis * 2 // Positive face
                };

                if t1 > tmin {
                    tmin = t1;
                    hit_face = face;
                }

                tmax = tmax.min(t2);
            } else {
                // Ray parallel to slab
                if origin < min_val || origin > max_val {
                    return Intersect::empty();
                }
            }

            if tmin > tmax {
                return Intersect::empty();
            }
        }

        if tmin < 0.0 {
            return Intersect::empty();
        }

        let point = *ray_origin + *ray_direction * tmin;
        let normal = Self::get_normal_for_face(hit_face);
        let uv = self.calculate_uv(&point, hit_face); // NEW: Calculate UV

        Intersect::new(
            self.materials[hit_face],
            true,
            tmin,
            point,
            normal,
            uv, // NEW: Pass UV
            hit_face,
        )
    }
}
