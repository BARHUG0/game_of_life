use crate::intersection::Intersect;
use crate::objects::{Object, RayIntersect};
use raylib::prelude::*;

// Axis-Aligned Bounding Box
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}

impl AABB {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        AABB { min, max }
    }

    // Create AABB from an object
    pub fn from_object(object: &Object) -> Self {
        match object {
            Object::Sphere(sphere) => {
                let center = sphere.center();
                let radius = sphere.radius();
                AABB {
                    min: Vector3::new(center.x - radius, center.y - radius, center.z - radius),
                    max: Vector3::new(center.x + radius, center.y + radius, center.z + radius),
                }
            }
            Object::Cube(cube) => AABB {
                min: cube.min,
                max: cube.max,
            },
        }
    }

    // Combine two AABBs
    pub fn union(&self, other: &AABB) -> AABB {
        AABB {
            min: Vector3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vector3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    // Get center point
    pub fn center(&self) -> Vector3 {
        Vector3::new(
            (self.min.x + self.max.x) * 0.5,
            (self.min.y + self.max.y) * 0.5,
            (self.min.z + self.max.z) * 0.5,
        )
    }

    // Get surface area (used for SAH)
    pub fn surface_area(&self) -> f32 {
        let d = self.max - self.min;
        2.0 * (d.x * d.y + d.y * d.z + d.z * d.x)
    }

    // Ray-AABB intersection test (slab method)
    pub fn intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> bool {
        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;

        // X axis
        if ray_direction.x.abs() > 1e-6 {
            let inv_d = 1.0 / ray_direction.x;
            let mut t1 = (self.min.x - ray_origin.x) * inv_d;
            let mut t2 = (self.max.x - ray_origin.x) * inv_d;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);
            if tmin > tmax {
                return false;
            }
        } else if ray_origin.x < self.min.x || ray_origin.x > self.max.x {
            return false;
        }

        // Y axis
        if ray_direction.y.abs() > 1e-6 {
            let inv_d = 1.0 / ray_direction.y;
            let mut t1 = (self.min.y - ray_origin.y) * inv_d;
            let mut t2 = (self.max.y - ray_origin.y) * inv_d;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);
            if tmin > tmax {
                return false;
            }
        } else if ray_origin.y < self.min.y || ray_origin.y > self.max.y {
            return false;
        }

        // Z axis
        if ray_direction.z.abs() > 1e-6 {
            let inv_d = 1.0 / ray_direction.z;
            let mut t1 = (self.min.z - ray_origin.z) * inv_d;
            let mut t2 = (self.max.z - ray_origin.z) * inv_d;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);
            if tmin > tmax {
                return false;
            }
        } else if ray_origin.z < self.min.z || ray_origin.z > self.max.z {
            return false;
        }

        tmin < tmax && tmax > 0.0
    }
}

// BVH Node
pub enum BVHNode {
    Leaf {
        bounds: AABB,
        object_index: usize,
    },
    Internal {
        bounds: AABB,
        left: Box<BVHNode>,
        right: Box<BVHNode>,
    },
}

impl BVHNode {
    pub fn bounds(&self) -> &AABB {
        match self {
            BVHNode::Leaf { bounds, .. } => bounds,
            BVHNode::Internal { bounds, .. } => bounds,
        }
    }
}

// Main BVH structure
pub struct BVH {
    root: BVHNode,
    objects: Vec<Object>,
}

impl BVH {
    // Build BVH from objects
    pub fn build(objects: Vec<Object>) -> Self {
        let object_count = objects.len();

        if object_count == 0 {
            panic!("Cannot build BVH with no objects");
        }

        // Create list of object indices with their bounding boxes and centroids
        let mut object_info: Vec<(usize, AABB, Vector3)> = objects
            .iter()
            .enumerate()
            .map(|(idx, obj)| {
                let bounds = AABB::from_object(obj);
                let centroid = bounds.center();
                (idx, bounds, centroid)
            })
            .collect();

        let root = Self::build_recursive(&mut object_info, 0, object_count);

        BVH { root, objects }
    }

    // Recursive BVH construction using SAH (Surface Area Heuristic)
    fn build_recursive(
        object_info: &mut [(usize, AABB, Vector3)],
        start: usize,
        end: usize,
    ) -> BVHNode {
        let count = end - start;

        // Leaf node: single object
        if count == 1 {
            let (object_index, bounds, _) = object_info[start];
            return BVHNode::Leaf {
                bounds,
                object_index,
            };
        }

        // Calculate bounding box for all objects in range
        let mut bounds = object_info[start].1;
        for i in (start + 1)..end {
            bounds = bounds.union(&object_info[i].1);
        }

        // For small counts, just split in middle
        if count <= 4 {
            let mid = start + count / 2;

            // Find best axis to split on
            let extent = bounds.max - bounds.min;
            let axis = if extent.x > extent.y && extent.x > extent.z {
                0 // X
            } else if extent.y > extent.z {
                1 // Y
            } else {
                2 // Z
            };

            // Sort by centroid along chosen axis
            object_info[start..end].sort_by(|a, b| {
                let a_val = match axis {
                    0 => a.2.x,
                    1 => a.2.y,
                    _ => a.2.z,
                };
                let b_val = match axis {
                    0 => b.2.x,
                    1 => b.2.y,
                    _ => b.2.z,
                };
                a_val.partial_cmp(&b_val).unwrap()
            });

            let left = Box::new(Self::build_recursive(object_info, start, mid));
            let right = Box::new(Self::build_recursive(object_info, mid, end));

            return BVHNode::Internal {
                bounds,
                left,
                right,
            };
        }

        // For larger counts, use SAH to find best split
        let (split_axis, split_pos) = Self::find_best_split(object_info, start, end, &bounds);

        // Partition objects based on split
        let mut i = start;
        let mut j = end - 1;

        while i <= j {
            let centroid_val = match split_axis {
                0 => object_info[i].2.x,
                1 => object_info[i].2.y,
                _ => object_info[i].2.z,
            };

            if centroid_val < split_pos {
                i += 1;
            } else {
                object_info.swap(i, j);
                if j == 0 {
                    break;
                }
                j -= 1;
            }
        }

        let mid = i;

        // Ensure we don't create empty nodes
        let mid = if mid == start {
            start + 1
        } else if mid == end {
            end - 1
        } else {
            mid
        };

        let left = Box::new(Self::build_recursive(object_info, start, mid));
        let right = Box::new(Self::build_recursive(object_info, mid, end));

        BVHNode::Internal {
            bounds,
            left,
            right,
        }
    }

    // Find best split using Surface Area Heuristic
    fn find_best_split(
        object_info: &[(usize, AABB, Vector3)],
        start: usize,
        end: usize,
        bounds: &AABB,
    ) -> (usize, f32) {
        const NUM_BUCKETS: usize = 12;

        let count = end - start;
        let mut best_cost = f32::INFINITY;
        let mut best_axis = 0;
        let mut best_split = 0.0;

        // Try each axis
        for axis in 0..3 {
            let bounds_min = match axis {
                0 => bounds.min.x,
                1 => bounds.min.y,
                _ => bounds.min.z,
            };
            let bounds_max = match axis {
                0 => bounds.max.x,
                1 => bounds.max.y,
                _ => bounds.max.z,
            };

            if (bounds_max - bounds_min).abs() < 1e-6 {
                continue;
            }

            // Initialize buckets
            let mut buckets = vec![(0, AABB::new(Vector3::zero(), Vector3::zero())); NUM_BUCKETS];

            // Place objects into buckets
            for i in start..end {
                let centroid_val = match axis {
                    0 => object_info[i].2.x,
                    1 => object_info[i].2.y,
                    _ => object_info[i].2.z,
                };

                let mut bucket_idx = ((centroid_val - bounds_min) / (bounds_max - bounds_min)
                    * NUM_BUCKETS as f32) as usize;
                bucket_idx = bucket_idx.min(NUM_BUCKETS - 1);

                if buckets[bucket_idx].0 == 0 {
                    buckets[bucket_idx].1 = object_info[i].1;
                } else {
                    buckets[bucket_idx].1 = buckets[bucket_idx].1.union(&object_info[i].1);
                }
                buckets[bucket_idx].0 += 1;
            }

            // Compute costs for each split position
            for split_bucket in 1..NUM_BUCKETS {
                let mut left_count = 0;
                let mut left_bounds: Option<AABB> = None;

                for i in 0..split_bucket {
                    if buckets[i].0 > 0 {
                        left_count += buckets[i].0;
                        left_bounds = Some(match left_bounds {
                            None => buckets[i].1,
                            Some(b) => b.union(&buckets[i].1),
                        });
                    }
                }

                let mut right_count = 0;
                let mut right_bounds: Option<AABB> = None;

                for i in split_bucket..NUM_BUCKETS {
                    if buckets[i].0 > 0 {
                        right_count += buckets[i].0;
                        right_bounds = Some(match right_bounds {
                            None => buckets[i].1,
                            Some(b) => b.union(&buckets[i].1),
                        });
                    }
                }

                if left_count == 0 || right_count == 0 {
                    continue;
                }

                let left_area = left_bounds.unwrap().surface_area();
                let right_area = right_bounds.unwrap().surface_area();

                let cost = 0.125
                    + (left_count as f32 * left_area + right_count as f32 * right_area)
                        / bounds.surface_area();

                if cost < best_cost {
                    best_cost = cost;
                    best_axis = axis;
                    let split_t = split_bucket as f32 / NUM_BUCKETS as f32;
                    best_split = bounds_min + (bounds_max - bounds_min) * split_t;
                }
            }
        }

        // If no good split found, split in middle
        if best_cost == f32::INFINITY {
            best_axis = 0;
            best_split = (bounds.min.x + bounds.max.x) * 0.5;
        }

        (best_axis, best_split)
    }

    // Traverse BVH and find closest intersection
    pub fn intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        self.traverse_node(&self.root, ray_origin, ray_direction, f32::INFINITY)
    }

    fn traverse_node(
        &self,
        node: &BVHNode,
        ray_origin: &Vector3,
        ray_direction: &Vector3,
        mut closest_distance: f32,
    ) -> Intersect {
        if !node.bounds().intersect(ray_origin, ray_direction) {
            return Intersect::empty();
        }

        match node {
            BVHNode::Leaf { object_index, .. } => {
                let object = &self.objects[*object_index];
                let intersection = object.ray_intersect(ray_origin, ray_direction);
                if intersection.is_intersecting() && intersection.distance() < closest_distance {
                    return intersection;
                }
                Intersect::empty()
            }
            BVHNode::Internal { left, right, .. } => {
                // Calculate distances to both children's AABBs
                let left_dist = Self::aabb_distance(left.bounds(), ray_origin, ray_direction);
                let right_dist = Self::aabb_distance(right.bounds(), ray_origin, ray_direction);

                // Order traversal by distance
                let (first, second) = if left_dist < right_dist {
                    (left, right)
                } else {
                    (right, left)
                };

                // Traverse nearest first
                let first_hit =
                    self.traverse_node(first, ray_origin, ray_direction, closest_distance);

                if first_hit.is_intersecting() {
                    closest_distance = first_hit.distance();
                }

                // Only traverse second if it could be closer
                let second_dist = if left_dist < right_dist {
                    right_dist
                } else {
                    left_dist
                };
                if second_dist < closest_distance {
                    let second_hit =
                        self.traverse_node(second, ray_origin, ray_direction, closest_distance);

                    if !first_hit.is_intersecting() {
                        return second_hit;
                    } else if second_hit.is_intersecting()
                        && second_hit.distance() < first_hit.distance()
                    {
                        return second_hit;
                    }
                }

                first_hit
            }
        }
    }

    // Helper function
    fn aabb_distance(aabb: &AABB, ray_origin: &Vector3, ray_direction: &Vector3) -> f32 {
        let mut tmin = f32::NEG_INFINITY;

        for axis in 0..3 {
            let origin = match axis {
                0 => ray_origin.x,
                1 => ray_origin.y,
                _ => ray_origin.z,
            };
            let direction = match axis {
                0 => ray_direction.x,
                1 => ray_direction.y,
                _ => ray_direction.z,
            };
            let min_val = match axis {
                0 => aabb.min.x,
                1 => aabb.min.y,
                _ => aabb.min.z,
            };
            let max_val = match axis {
                0 => aabb.max.x,
                1 => aabb.max.y,
                _ => aabb.max.z,
            };

            if direction.abs() > 1e-6 {
                let inv_d = 1.0 / direction;
                let mut t1 = (min_val - origin) * inv_d;
                let mut t2 = (max_val - origin) * inv_d;
                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                }
                tmin = tmin.max(t1);
            }
        }

        tmin.max(0.0)
    }

    // Get reference to objects (for shadow rays, emissive collection, etc.)
    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
}
