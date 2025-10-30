#![allow(warnings)]

mod framebuffer;
mod light;
mod line;
mod matrix;
mod obj;
mod render;
mod triangle;
mod vertex;

use framebuffer::Framebuffer;
use obj::Obj;
use raylib::prelude::*;
use render::{Camera, render_model};
use std::f32::consts::PI;

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

    let obj = Obj::load("models/nave.obj").expect("Failed to load Obj");
    let vertex_array = obj.get_vertex_array();

    println!(
        "Vertices: {}, Indices: {}",
        obj.vertices().len(),
        obj.indices().len()
    );

    framebuffer.set_background_color(Color::new(4, 12, 36, 255));
    framebuffer.set_foreground_color(Color::new(100, 200, 255, 255));

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 5.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 3.0,
        WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
    );

    let mut translation = Vector3::new(0.0, 0.0, 0.0);
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);
    let mut scale = 1.0;

    while !&handle.window_should_close() {
        framebuffer.clear();

        // Input handling - Translation
        if handle.is_key_down(KeyboardKey::KEY_A) {
            translation.x += 0.05;
        }
        if handle.is_key_down(KeyboardKey::KEY_D) {
            translation.x -= 0.05;
        }
        if handle.is_key_down(KeyboardKey::KEY_W) {
            translation.z += 0.05;
        }
        if handle.is_key_down(KeyboardKey::KEY_S) {
            translation.z -= 0.05;
        }
        if handle.is_key_down(KeyboardKey::KEY_Q) {
            translation.y += 0.05;
        }
        if handle.is_key_down(KeyboardKey::KEY_E) {
            translation.y -= 0.05;
        }

        // Rotation
        if handle.is_key_down(KeyboardKey::KEY_LEFT) {
            rotation.y += 0.02;
        }
        if handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            rotation.y -= 0.02;
        }
        if handle.is_key_down(KeyboardKey::KEY_UP) {
            rotation.x += 0.02;
        }
        if handle.is_key_down(KeyboardKey::KEY_DOWN) {
            rotation.x -= 0.02;
        }

        // Scale
        if handle.is_key_down(KeyboardKey::KEY_U) {
            scale += 0.01;
        }
        if handle.is_key_down(KeyboardKey::KEY_J) {
            scale -= 0.01;
        }

        // Render the model
        render_model(
            &mut framebuffer,
            &vertex_array,
            translation,
            scale,
            rotation,
            &camera,
        );

        let texture = handle
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .expect("The texture loaded from the color buffer should be valid");

        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            draw_handle.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}
