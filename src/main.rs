#![allow(warnings)]

mod conway;
mod framebuffer;
mod raytracer;

use rand::Rng;
use raylib::prelude::*;

use std::thread;
use std::time::Duration;

use framebuffer::Framebuffer;

use raytracer::{Material, Sphere, render};

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
        Sphere::new(
            Vector3::new(-2.0, 0.0, -5.0),
            1.0,
            Material::new(Color::GREENYELLOW),
        ),
        Sphere::new(Vector3::new(2.0, 0.0, -5.0), 1.5, Material::new(Color::RED)),
    ];

    while !&handle.window_should_close() {
        framebuffer.clear();

        render(&mut framebuffer, &objects);

        let texture = handle
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .expect("The texture loaded from the color buffer should be valid");

        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            draw_handle.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}
