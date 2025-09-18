#![allow(warnings)]

mod camera;
mod conway;
mod framebuffer;
mod material;
mod raytracer;

use rand::Rng;
use raylib::prelude::*;

use std::thread;
use std::time::Duration;

use framebuffer::Framebuffer;

use camera::Camera;
use material::Material;
use raytracer::{Cube, Sphere, render_with_camera};

const WINDOW_WIDTH: i32 = 1900;
const WINDOW_HEIGHT: i32 = 1000;

const FREMEBUFFER_WIDTH: i32 = WINDOW_WIDTH;
const FRAMEBUFFER_HEIGHT: i32 = WINDOW_HEIGHT;

fn main() {
    game_loop();
}

fn game_loop() {
    let (mut handle, raylib_thread) = raylib::init()
        .undecorated()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("raylib")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(FREMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT, Color::WHITE);

    framebuffer.set_background_color(Color::new(80, 80, 200, 255));

    let objects = [
        Sphere::new(Vector3::new(1.0, 0.0, -4.0), 1.0, Material::IVORY()),
        Sphere::new(Vector3::new(2.0, 0.0, -5.0), 1.0, Material::RUBBER()),
        Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0, Material::RUBBER()),
    ];

    let snowman = [
        Sphere::new(Vector3::new(0.0, -2.0, -8.0), 2.0, Material::IVORY()), // bottom
        Sphere::new(Vector3::new(0.0, 1.0, -8.0), 1.4, Material::IVORY()),  // middle
        Sphere::new(Vector3::new(0.0, 3.2, -8.0), 1.0, Material::IVORY()),  // head
        Sphere::new(Vector3::new(-0.35, 3.4, -7.2), 0.15, Material::RUBBER()), // left eye
        Sphere::new(Vector3::new(0.35, 3.4, -7.2), 0.15, Material::RUBBER()), // right eye
        Sphere::new(Vector3::new(0.0, 1.5, -6.8), 0.18, Material::RUBBER()), // top button
        Sphere::new(Vector3::new(0.0, 1.0, -6.7), 0.18, Material::RUBBER()), // middle button
        Sphere::new(Vector3::new(0.0, 0.5, -6.8), 0.18, Material::RUBBER()), // bottom button
    ];

    let cube = Cube::new(
        Vector3::new(-1.0, -1.0, -1.0),
        Vector3::new(1.0, 1.0, 1.0),
        [
            Material::new(Color::RED),    // +X
            Material::new(Color::BLUE),   // -X
            Material::new(Color::GREEN),  // +Y
            Material::new(Color::YELLOW), // -Y
            Material::new(Color::ORANGE), // +Z
            Material::new(Color::PURPLE), // -Z
        ],
    );

    let cubes = [cube];

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let rotation_speed = PI as f32 / 100.0;

    while !&handle.window_should_close() {
        framebuffer.clear();

        if handle.is_key_down(KeyboardKey::KEY_LEFT) {
            camera.orbit(rotation_speed, 0.0);
        }
        if handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if handle.is_key_down(KeyboardKey::KEY_UP) {
            camera.orbit(0.0, -rotation_speed);
        }
        if handle.is_key_down(KeyboardKey::KEY_DOWN) {
            camera.orbit(0.0, rotation_speed);
        }

        render_with_camera(&mut framebuffer, &cubes, &camera);

        let texture = handle
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .expect("The texture loaded from the color buffer should be valid");

        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            draw_handle.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}
