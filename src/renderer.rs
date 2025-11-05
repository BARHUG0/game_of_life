use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::intersection::Intersect;
use crate::light::Light;
use crate::objects::{Object, RayIntersect};
use raylib::prelude::*;

const BACKGROUND_COLOR: Color = Color::new(4, 12, 36, 255);

pub fn render(framebuffer: &mut Framebuffer, objects: &[Object], camera: &Camera, light: &Light) {
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

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light);

            framebuffer.set_foreground_color(pixel_color);
            framebuffer.set_pixel(x, y);
        }
    }
}

pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[Object],
    light: &Light,
) -> Color {
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
        return BACKGROUND_COLOR;
    }

    // Calculate lighting
    let light_dir = (light.position - intersection.point).normalized();
    let normal = intersection.normal;
    let material = intersection.material();

    // Diffuse lighting: intensity * max(0, N · L)
    let diffuse_intensity = normal.dot(light_dir).max(0.0) * light.intensity;

    // Ambient lighting (so dark sides aren't pure black)
    let ambient = 0.2;

    // Combine ambient + diffuse
    let total_intensity = (ambient + diffuse_intensity * material.albedo).min(1.0);

    // Apply intensity to material color
    let final_color = color_multiply(material.diffuse, total_intensity);

    final_color
}

fn color_multiply(color: Color, intensity: f32) -> Color {
    Color::new(
        (color.r as f32 * intensity) as u8,
        (color.g as f32 * intensity) as u8,
        (color.b as f32 * intensity) as u8,
        color.a,
    )
}
