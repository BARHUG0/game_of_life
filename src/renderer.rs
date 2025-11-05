use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::intersection::Intersect;
use crate::objects::{Object, RayIntersect};
use raylib::prelude::*;

const BACKGROUND_COLOR: Color = Color::new(4, 12, 36, 255);

pub fn render(framebuffer: &mut Framebuffer, objects: &[Object], camera: &Camera) {
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

pub fn cast_ray(ray_origin: &Vector3, ray_direction: &Vector3, objects: &[Object]) -> Color {
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

    intersection.material().diffuse_color()
}
