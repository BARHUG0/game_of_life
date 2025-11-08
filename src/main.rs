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
use light::Light;
use material::Material;
use objects::{Cube, Object, Sphere};
use raylib::prelude::*;
use renderer::render;

const WINDOW_WIDTH: i32 = 1900;
const WINDOW_HEIGHT: i32 = 1000;

const FRAMEBUFFER_WIDTH: i32 = WINDOW_WIDTH;
const FRAMEBUFFER_HEIGHT: i32 = WINDOW_HEIGHT;

// LIGHT CONFIGURATION
const NUM_LIGHTS: usize = 3;

// Light 0 - White key light
const LIGHT_0_POSITION: Vector3 = Vector3 {
    x: 3.0,
    y: 5.0,
    z: 5.0,
};
const LIGHT_0_INTENSITY: f32 = 8.0;
const LIGHT_0_COLOR: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

// Light 1 - Red fill light
const LIGHT_1_POSITION: Vector3 = Vector3 {
    x: -4.0,
    y: 2.0,
    z: 3.0,
};
const LIGHT_1_INTENSITY: f32 = 4.0;
const LIGHT_1_COLOR: Color = Color {
    r: 255,
    g: 100,
    b: 100,
    a: 255,
};

// Light 2 - Blue rim light
const LIGHT_2_POSITION: Vector3 = Vector3 {
    x: 0.0,
    y: -3.0,
    z: 6.0,
};
const LIGHT_2_INTENSITY: f32 = 4.0;
const LIGHT_2_COLOR: Color = Color {
    r: 100,
    g: 150,
    b: 255,
    a: 255,
};

fn main() {
    game_loop();
}

fn game_loop() {
    let (mut handle, raylib_thread) = raylib::init()
        .undecorated()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Raytracer - Shadows & Reflections")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT, Color::WHITE);

    framebuffer.set_background_color(Color::new(80, 80, 200, 255));

    // Create a colorful cube (non-reflective)
    let cube = Cube::new(
        Vector3::new(-1.0, -1.0, -1.0),
        Vector3::new(1.0, 1.0, 1.0),
        [
            Material::simple(Color::RED),    // +X
            Material::simple(Color::BLUE),   // -X
            Material::simple(Color::GREEN),  // +Y
            Material::simple(Color::YELLOW), // -Y
            Material::simple(Color::ORANGE), // +Z
            Material::simple(Color::PURPLE), // -Z
        ],
    );

    // Create a metallic sphere to the left
    let sphere_metal = Sphere::new(
        Vector3::new(-3.0, 0.0, 0.0),
        1.0,
        Material::METAL(Color::new(180, 180, 200, 255)),
    );

    // Create a mirror sphere to the right
    let sphere_mirror = Sphere::new(Vector3::new(3.0, 0.0, 0.0), 1.0, Material::MIRROR());

    // Create a small glass-like sphere in front

    let sphere_glass = Sphere::new(
        Vector3::new(0.0, 2.0, 2.0),
        0.6,
        Material::GLASS(), // Now has proper refraction!
    );

    let objects = vec![
        Object::Cube(cube),
        Object::Sphere(sphere_metal),
        Object::Sphere(sphere_mirror),
        Object::Sphere(sphere_glass),
    ];

    let mut lights = vec![
        Light::new(LIGHT_0_POSITION, LIGHT_0_INTENSITY, LIGHT_0_COLOR),
        Light::new(LIGHT_1_POSITION, LIGHT_1_INTENSITY, LIGHT_1_COLOR),
        Light::new(LIGHT_2_POSITION, LIGHT_2_INTENSITY, LIGHT_2_COLOR),
    ];

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let rotation_speed = PI as f32 / 100.0;
    let light_move_speed = 0.5;
    let mut selected_light: usize = 0;

    println!("=== Controls ===");
    println!("Arrow Keys: Rotate camera");
    println!("1/2/3: Select light source");
    println!("WASD + Q/E: Move selected light");
    println!("================");

    while !&handle.window_should_close() {
        framebuffer.clear();

        // Light selection (1, 2, 3 keys)
        if handle.is_key_pressed(KeyboardKey::KEY_ONE) {
            selected_light = 0;
            println!(
                "Selected Light 0 (White) - Intensity: {}",
                LIGHT_0_INTENSITY
            );
        }
        if handle.is_key_pressed(KeyboardKey::KEY_TWO) {
            selected_light = 1;
            println!("Selected Light 1 (Red) - Intensity: {}", LIGHT_1_INTENSITY);
        }
        if handle.is_key_pressed(KeyboardKey::KEY_THREE) {
            selected_light = 2;
            println!("Selected Light 2 (Blue) - Intensity: {}", LIGHT_2_INTENSITY);
        }

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

        // Light controls (WASD + Q/E)
        if handle.is_key_down(KeyboardKey::KEY_W) {
            lights[selected_light].position.z -= light_move_speed;
        }
        if handle.is_key_down(KeyboardKey::KEY_S) {
            lights[selected_light].position.z += light_move_speed;
        }
        if handle.is_key_down(KeyboardKey::KEY_A) {
            lights[selected_light].position.x -= light_move_speed;
        }
        if handle.is_key_down(KeyboardKey::KEY_D) {
            lights[selected_light].position.x += light_move_speed;
        }
        if handle.is_key_down(KeyboardKey::KEY_Q) {
            lights[selected_light].position.y -= light_move_speed;
        }
        if handle.is_key_down(KeyboardKey::KEY_E) {
            lights[selected_light].position.y += light_move_speed;
        }

        render(&mut framebuffer, &objects, &camera, &lights);

        let texture = handle
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .expect("The texture loaded from the color buffer should be valid");

        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            draw_handle.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}
