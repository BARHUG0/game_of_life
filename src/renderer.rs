use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::intersection::Intersect;
use crate::light::Light;
use crate::objects::{Object, RayIntersect};
use raylib::prelude::*;

const BACKGROUND_COLOR: Color = Color::new(4, 12, 36, 255);

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Object],
    camera: &Camera,
    lights: &[Light],
) {
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

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, lights);

            framebuffer.set_foreground_color(pixel_color);
            framebuffer.set_pixel(x, y);
        }
    }
}

pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[Object],
    lights: &[Light],
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

    let view_dir = (*ray_origin - intersection.point).normalized();
    let normal = intersection.normal;
    let material = intersection.material();

    // Ambient lighting
    let ambient = 0.1;
    let mut diffuse_color = color_multiply(material.diffuse, ambient);
    let mut specular_color = Color::new(0, 0, 0, 255);

    // Accumulate lighting from all light sources
    for light in lights {
        let light_dir = (light.position - intersection.point).normalized();

        // Get attenuated light intensity based on distance
        let light_intensity = light.get_intensity_at(&intersection.point);

        // Diffuse lighting
        let diffuse_intensity = normal.dot(light_dir).max(0.0) * light_intensity;
        let diffuse_contribution =
            color_multiply(light.color, diffuse_intensity * material.albedo[0]);
        diffuse_color = color_add(diffuse_color, diffuse_contribution);

        // Specular lighting
        let reflect_dir = reflect(&light_dir, &normal);
        let specular_intensity =
            view_dir.dot(reflect_dir).max(0.0).powf(material.specular) * light_intensity;
        let specular_contribution =
            color_multiply(light.color, specular_intensity * material.albedo[1]);
        specular_color = color_add(specular_color, specular_contribution);
    }

    // Combine diffuse + specular
    color_add(diffuse_color, specular_color)
}

fn color_multiply(color: Color, intensity: f32) -> Color {
    Color::new(
        (color.r as f32 * intensity).min(255.0) as u8,
        (color.g as f32 * intensity).min(255.0) as u8,
        (color.b as f32 * intensity).min(255.0) as u8,
        color.a,
    )
}

fn color_add(a: Color, b: Color) -> Color {
    Color::new(
        (a.r as u16 + b.r as u16).min(255) as u8,
        (a.g as u16 + b.g as u16).min(255) as u8,
        (a.b as u16 + b.b as u16).min(255) as u8,
        a.a,
    )
}

fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    // R = I - 2(N · I)N
    *incident - *normal * 2.0 * normal.dot(*incident)
}
