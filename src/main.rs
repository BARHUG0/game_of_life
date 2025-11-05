#![allow(warnings)]

mod camera;
mod framebuffer;
mod intersection;
mod light;
mod material;
mod objects;
mod renderer;

use camera::Camera;
use framebuffer::Framebuffer;
use material::Material;
use objects::{Cube, Object, Sphere};
use raylib::prelude::*;
use renderer::render;

const WINDOW_WIDTH: i32 = 1900;
const WINDOW_HEIGHT: i32 = 1000;

const FRAMEBUFFER_WIDTH: i32 = WINDOW_WIDTH;
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

    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT, Color::WHITE);

    framebuffer.set_background_color(Color::new(80, 80, 200, 255));

    // Create cube with per-face materials
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

    // Using the enum-based Object system
    let objects = vec![Object::Cube(cube)];

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let rotation_speed = PI as f32 / 100.0;

    while !&handle.window_should_close() {
        framebuffer.clear();

        // Camera controls
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

        render(&mut framebuffer, &objects, &camera);

        let texture = handle
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .expect("The texture loaded from the color buffer should be valid");

        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            draw_handle.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}
