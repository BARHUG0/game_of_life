#![allow(warnings)]

mod fragment;
mod framebuffer;
mod light;
mod line;
mod matrix;
mod obj;
mod render;
mod shader;
mod triangle;
mod uniforms;
mod vertex;

use framebuffer::Framebuffer;
use obj::Obj;
use raylib::prelude::*;
use render::{Camera, RenderMode, render_model};
use shader::ShaderType;
use std::f32::consts::PI;
use uniforms::Uniforms;

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

    let obj = Obj::load("models/sphere.obj").expect("Failed to load Obj");
    let vertex_array = obj.get_vertex_array();

    println!(
        "Vertices: {}, Indices: {}",
        obj.vertices().len(),
        obj.indices().len()
    );

    framebuffer.set_background_color(Color::new(10, 10, 30, 255));
    framebuffer.set_foreground_color(Color::new(100, 200, 255, 255));

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 15.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        PI / 3.0,
        WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
    );

    let mut translation = Vector3::new(0.0, 0.0, 0.0);
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);
    let mut scale = 1.0;
    let mut render_mode = RenderMode::Solid;
    let mut current_shader = ShaderType::None;
    let mut time = 0.0f32;

    // Draw UI text
    let shader_name = match current_shader {
        ShaderType::Rocky => "Planeta Rocoso",
        ShaderType::GasGiant => "Gigante Gaseoso",
        ShaderType::Ringed => "Planeta con anillo",
        ShaderType::Magenta => "Gigante Magenta (GJ 504 b)",
        ShaderType::WaterWorld => "Mundo de Agua (Kepler-22 b)",
        ShaderType::None => "Ninguno",
    };

    while !&handle.window_should_close() {
        framebuffer.clear();

        // Update time
        time += 0.016; // Approximately 60 FPS

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

        // Render mode toggle
        if handle.is_key_pressed(KeyboardKey::KEY_ONE) {
            render_mode = RenderMode::Wireframe;
        }
        if handle.is_key_pressed(KeyboardKey::KEY_TWO) {
            render_mode = RenderMode::Solid;
        }

        // Shader selection
        if handle.is_key_pressed(KeyboardKey::KEY_THREE) {
            current_shader = ShaderType::Rocky;
            println!("Switched to Rocky Planet Shader");
        }
        if handle.is_key_pressed(KeyboardKey::KEY_FOUR) {
            current_shader = ShaderType::GasGiant;
            println!("Switched to Gas Giant Shader");
        }
        if handle.is_key_pressed(KeyboardKey::KEY_FIVE) {
            current_shader = ShaderType::Ringed;
            println!("Switched to Ringed Planet Shader");
        }
        if handle.is_key_pressed(KeyboardKey::KEY_ZERO) {
            current_shader = ShaderType::None;
            println!("No shader - using solid color");
        }
        if handle.is_key_pressed(KeyboardKey::KEY_SIX) {
            current_shader = ShaderType::Magenta;
            println!("Switched to Magenta Gas Giant Shader (GJ 504 b)");
        }
        if handle.is_key_pressed(KeyboardKey::KEY_SEVEN) {
            current_shader = ShaderType::WaterWorld;
            println!("Switched to Water World Shader (Kepler-22 b)");
        }

        // Create uniforms
        let light_direction = Vector3::new(0.0, 0.0, 1.0); // Light coming from camera direction
        let uniforms = Uniforms::new(time, light_direction, camera.position());

        // Render the model
        render_model(
            &mut framebuffer,
            &vertex_array,
            translation,
            scale,
            rotation,
            &camera,
            render_mode,
            current_shader,
            &uniforms,
        );

        let texture = handle
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .expect("The texture loaded from the color buffer should be valid");

        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            draw_handle.clear_background(Color::BLACK);
            draw_handle.draw_texture(&texture, 0, 0, Color::WHITE);

            draw_handle.draw_text(
                &format!("Shader: {} (0/3/4/5/6/7)", shader_name),
                10,
                10,
                20,
                Color::WHITE,
            );

            draw_handle.draw_text("Modo: 1-Wireframe, 2-Solido", 10, 35, 20, Color::WHITE);
            draw_handle.draw_text(
                "Movimiento: WASD, Q/E, Rotacion: Flechas, Escala: U/J ",
                10,
                60,
                20,
                Color::WHITE,
            );
        }
    }
}
