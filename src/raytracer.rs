use std::{f32, mem::zeroed};

use crate::camera::Camera;
use crate::material::Material;
use crate::{Framebuffer, framebuffer};
use raylib::prelude::*;

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect;
}

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
                    return Intersect {
                        material: self.materials[0],
                        is_intersecting: false,
                        distance: f32::INFINITY,
                    };
                }
            } else {
                let inv_d = 1.0 / direction;
                let mut t1 = (min_val - origin) * inv_d;
                let mut t2 = (max_val - origin) * inv_d;

                let face_in;
                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                    face_in = 1; // -X
                } else {
                    face_in = 0; // +X
                }

                if t1 > tmin {
                    tmin = t1;
                    hit_face = Some(face_in);
                }
                if t2 < tmax {
                    tmax = t2;
                }
                if tmin > tmax {
                    return Intersect {
                        material: self.materials[0],
                        is_intersecting: false,
                        distance: f32::INFINITY,
                    };
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
                    return Intersect {
                        material: self.materials[0],
                        is_intersecting: false,
                        distance: f32::INFINITY,
                    };
                }
            } else {
                let inv_d = 1.0 / direction;
                let mut t1 = (min_val - origin) * inv_d;
                let mut t2 = (max_val - origin) * inv_d;

                let face_in;
                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                    face_in = 3; // -Y
                } else {
                    face_in = 2; // +Y
                }

                if t1 > tmin {
                    tmin = t1;
                    hit_face = Some(face_in);
                }
                if t2 < tmax {
                    tmax = t2;
                }
                if tmin > tmax {
                    return Intersect {
                        material: self.materials[0],
                        is_intersecting: false,
                        distance: f32::INFINITY,
                    };
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
                    return Intersect {
                        material: self.materials[0],
                        is_intersecting: false,
                        distance: f32::INFINITY,
                    };
                }
            } else {
                let inv_d = 1.0 / direction;
                let mut t1 = (min_val - origin) * inv_d;
                let mut t2 = (max_val - origin) * inv_d;

                let face_in;
                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                    face_in = 5; // -Z
                } else {
                    face_in = 4; // +Z
                }

                if t1 > tmin {
                    tmin = t1;
                    hit_face = Some(face_in);
                }
                if t2 < tmax {
                    tmax = t2;
                }
                if tmin > tmax {
                    return Intersect {
                        material: self.materials[0],
                        is_intersecting: false,
                        distance: f32::INFINITY,
                    };
                }
            }
        }

        if tmin < 0.0 {
            return Intersect {
                material: self.materials[0],
                is_intersecting: false,
                distance: f32::INFINITY,
            };
        }

        Intersect {
            material: self.materials[hit_face.unwrap_or(0)],
            is_intersecting: true,
            distance: tmin,
        }
    }
}

pub struct Sphere {
    center: Vector3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, material: Material) -> Sphere {
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

        let is_intersecting = discriminant > 0.0;

        let sqrt_disc = discriminant.sqrt();
        let t1 = (-b - sqrt_disc) / (2.0 * a);
        let t2 = (-b + sqrt_disc) / (2.0 * a);

        let distance = if t1 > 0.0 {
            t1
        } else if t2 > 0.0 {
            t2
        } else {
            f32::INFINITY // or None
        };

        Intersect::new(self.material, is_intersecting, distance)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    material: Material,
    is_intersecting: bool,
    distance: f32,
}

impl Intersect {
    pub fn new(material: Material, is_intersecting: bool, distance: f32) -> Self {
        Intersect {
            is_intersecting,
            material,
            distance,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            material: Material {
                diffuse_color: Color::new(0, 0, 0, 0),
            },
            is_intersecting: false,
            distance: 0.0,
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

/*
pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width() as f32;
    let height = framebuffer.height() as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio;

            let ray_direction = Vector3::new(screen_x, screen_y, -1.0).normalized();

            let pixel_color = cast_ray(&Vector3::zero(), &ray_direction, objects);

            framebuffer.set_foreground_color(pixel_color);
            framebuffer.set_pixel(x, y);
        }
    }
}
*/

pub fn render_with_camera(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera) {
    let width = framebuffer.width() as f32;
    let height = framebuffer.height() as f32;

    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan() as f32;

    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = Vector3::new(screen_x, screen_y, -1.0).normalized();

            let rotated_direction = camera.basis_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects);

            framebuffer.set_foreground_color(pixel_color);
            framebuffer.set_pixel(x, y);
        }
    }
}

pub fn cast_ray(ray_origin: &Vector3, ray_direction: &Vector3, objects: &[Cube]) -> Color {
    let mut intersection = Intersect::empty();

    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting() && tmp.distance() < zbuffer {
            zbuffer = tmp.distance;
            intersection = tmp;
        }
    }

    if !intersection.is_intersecting {
        return Color::new(4, 12, 36, 255);
    }

    intersection.material().diffuse_color()
}
