#![allow(warnings)]

mod fragment;
mod framebuffer;
mod light;
mod line;
mod matrix;
mod obj;
mod orbital_math;
mod orbital_renderer;
mod render;
mod shader;
mod solar_system;
mod spaceship;
mod texture_manager;
mod triangle;
mod uniforms;
mod vertex;

use framebuffer::Framebuffer;
use obj::Obj;
use orbital_renderer::render_solar_system_orbits; // Add with other uses
use raylib::prelude::*;
use render::{Camera, RenderMode, render_model};
use shader::ShaderType;
use solar_system::{CentralBody, SolarSystem};
use spaceship::Spaceship;
use std::f32::consts::PI;
use texture_manager::TextureData;
use uniforms::Uniforms;
use vertex::Vertex;

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

    let torus_obj = Obj::load("models/torus.obj").expect("Failed to load torus");
    let torus_vertex_array = torus_obj.get_vertex_array();

    let spaceship_obj = Obj::load("models/nave.obj").expect("Failed to load spaceship model");
    let spaceship_vertex_array = spaceship_obj.get_vertex_array();

    // Calculate bounding box center
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    let mut min_z = f32::MAX;
    let mut max_z = f32::MIN;

    for vertex in &spaceship_vertex_array {
        let pos = vertex.position();
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
        min_z = min_z.min(pos.z);
        max_z = max_z.max(pos.z);
    }

    let center_offset = Vector3::new(
        -(min_x + max_x) / 2.0,
        -(min_y + max_y) / 2.0,
        -(min_z + max_z) / 2.0,
    );
    // Rotate vertices 90° on Y-axis to fix model orientation
    let rotation_angle = PI / 2.0;
    let (sin_y, cos_y) = rotation_angle.sin_cos();

    let centered_spaceship_vertices: Vec<Vertex> = spaceship_vertex_array
        .iter()
        .map(|vertex| {
            let pos = vertex.position();

            // Center the vertex
            let centered_pos = Vector3::new(
                pos.x + center_offset.x,
                pos.y + center_offset.y,
                pos.z + center_offset.z,
            );

            // Rotate 90° on Y-axis
            let rotated_pos = Vector3::new(
                centered_pos.x * cos_y + centered_pos.z * sin_y,
                centered_pos.y,
                -centered_pos.x * sin_y + centered_pos.z * cos_y,
            );

            // Also rotate the normal
            let normal = vertex.normal();
            let rotated_normal = Vector3::new(
                normal.x * cos_y + normal.z * sin_y,
                normal.y,
                -normal.x * sin_y + normal.z * cos_y,
            );

            Vertex::new(rotated_pos, rotated_normal, vertex.tex_coords())
        })
        .collect();
    // Create centered vertices

    let mut spaceship = Spaceship::new(Vector3::new(0.0, 5.0, 30.0), 0.3);

    // Initialize camera ONCE - following spaceship
    let mut camera = Camera::new(
        spaceship.get_camera_position(15.0, 5.0),
        spaceship.get_camera_target(2.0),
        spaceship.get_up_vector(),
        PI / 3.0,
        WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
    );

    let mut time = 0.0f32;
    let mut paused = false;
    let mut camera_mode = 0;

    // Load textures (keep your existing code)
    let mut noise_image_0 =
        Image::load_image("textures/perlin_noise.png").expect("Failed to load noise texture 0");
    let noise_texture_0 = TextureData::from_image(&mut noise_image_0);

    let noise_texture_1 = if let Ok(mut img) = Image::load_image("textures/accretion_noise.png") {
        TextureData::from_image(&mut img)
    } else {
        panic!("Missing texture_1")
    };

    let noise_texture_2 = if let Ok(mut img) = Image::load_image("textures/turbulence_noise.png") {
        TextureData::from_image(&mut img)
    } else {
        panic!("Missing texture_2")
    };

    framebuffer.set_background_color(Color::DARKBLUE);
    framebuffer.set_foreground_color(Color::new(100, 200, 255, 255));

    // CREATE SOLAR SYSTEM
    let mut solar_system = SolarSystem::new(
        CentralBody::BlackHole {
            position: Vector3::zero(),
            scale: 1.4,
        },
        1.5, // Distance ratio
    );

    // Add some planets
    solar_system.add_planet(ShaderType::Rocky, 0.5, 0.0, 0.0); // Rocky planet, slight ellipse
    solar_system.add_planet(ShaderType::GasGiant, 0.8, 0.0, 0.00); // Gas giant
    solar_system.add_planet(ShaderType::WaterWorld, 0.6, 0.00, 0.00); // Water world
    solar_system.add_planet(ShaderType::Magenta, 0.7, 0.0, 0.00); // Magenta giant

    let mut time = 0.0f32;
    let mut paused = false;

    println!("Ship pos: {:?}", spaceship.position);
    println!("Camera pos: {:?}", camera.position());
    println!("Camera target: {:?}", camera.target());

    while !&handle.window_should_close() {
        framebuffer.clear();

        let dt = if paused { 0.0 } else { 0.016 };
        time += dt;

        let mut forward = 0.0;
        let mut strafe = 0.0;
        let mut pitch = 0.0;
        let mut yaw = 0.0;

        if handle.is_key_down(KeyboardKey::KEY_W) {
            forward = 1.0;
        }
        if handle.is_key_down(KeyboardKey::KEY_S) {
            forward = -1.0;
        }
        if handle.is_key_down(KeyboardKey::KEY_A) {
            strafe = -1.0;
        }
        if handle.is_key_down(KeyboardKey::KEY_D) {
            strafe = 1.0;
        }
        if handle.is_key_down(KeyboardKey::KEY_UP) {
            pitch = -1.0;
        }
        if handle.is_key_down(KeyboardKey::KEY_DOWN) {
            pitch = 1.0;
        }
        if handle.is_key_down(KeyboardKey::KEY_LEFT) {
            yaw = -1.0;
        }
        if handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            yaw = 1.0;
        }

        println!(
            "Inputs: forward={}, strafe={}, pitch={}, yaw={}",
            forward, strafe, pitch, yaw
        );
        // Update spaceship physics
        spaceship.update(dt, forward, strafe, pitch, yaw);

        camera = Camera::new(
            spaceship.get_camera_position(15.0, 5.0),
            spaceship.get_camera_target(2.0),
            spaceship.get_up_vector(),
            PI / 3.0,
            WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
        );

        println!(
            "Camera pos: {:?}, target: {:?}",
            camera.position(),
            camera.target()
        );

        if handle.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }

        // Add planet
        if handle.is_key_pressed(KeyboardKey::KEY_P) {
            solar_system.add_planet(ShaderType::Ringed, 0.6, 0.25, 0.12);
        }

        // Remove last planet
        if handle.is_key_pressed(KeyboardKey::KEY_R) {
            if solar_system.planets.len() > 0 {
                solar_system.remove_planet(solar_system.planets.len() - 1);
            }
        }

        // Swap central body
        if handle.is_key_pressed(KeyboardKey::KEY_C) {
            solar_system.set_center(CentralBody::Star {
                position: Vector3::zero(),
                scale: 1.2,
                shader: ShaderType::GasGiant,
            });
        }

        if handle.is_key_pressed(KeyboardKey::KEY_B) {
            solar_system.set_center(CentralBody::BlackHole {
                position: Vector3::zero(),
                scale: 1.4,
            });
        }

        if handle.is_key_pressed(KeyboardKey::KEY_N) {
            solar_system.set_center(CentralBody::Binary {
                separation: 3.0,
                angle: 0.0,
                scale: 0.8,
                shader: ShaderType::Rocky,
            });
        }

        if handle.is_key_pressed(KeyboardKey::KEY_O) {
            solar_system.toggle_orbital_paths();
        }

        // Toggle trails
        if handle.is_key_pressed(KeyboardKey::KEY_T) {
            solar_system.toggle_trails();
        }

        // UPDATE SOLAR SYSTEM
        solar_system.update(dt);

        // After solar_system.update(dt);
        if solar_system.planets.len() > 0 {
            let planet = &solar_system.planets[0];
        }

        // In main.rs, after solar_system.update(dt);
        // RENDER
        let light_direction = Vector3::new(0.0, 0.0, 1.0);
        let uniforms = Uniforms::new_with_textures(
            time,
            light_direction,
            camera.position(),
            Some(&noise_texture_0),
            Some(&noise_texture_1),
            Some(&noise_texture_2),
        );

        render_starfield(
            &mut framebuffer.color_buffer,
            FRAMEBUFFER_WIDTH,
            FRAMEBUFFER_HEIGHT,
            Color::new(255, 255, 255, 255), // White stars
        );

        render_solar_system_orbits(&mut framebuffer, &solar_system, &camera);

        // Render central body
        match &solar_system.center {
            CentralBody::BlackHole { position, scale } => {
                // Render accretion disk first (background)
                render_model(
                    &mut framebuffer,
                    &torus_vertex_array,
                    *position,
                    *scale * 1.5,
                    render::Rotation::Euler(Vector3::zero()),
                    &camera,
                    RenderMode::Solid,
                    ShaderType::AccretionDisk,
                    &uniforms,
                );

                // Render black hole sphere
                render_model(
                    &mut framebuffer,
                    &vertex_array,
                    *position,
                    *scale,
                    render::Rotation::Euler(Vector3::zero()),
                    &camera,
                    RenderMode::Solid,
                    ShaderType::BlackHole,
                    &uniforms,
                );

                // Render disk again (foreground/lensing effect)
                render_model(
                    &mut framebuffer,
                    &torus_vertex_array,
                    *position,
                    *scale * 1.5,
                    render::Rotation::Euler(Vector3::zero()),
                    &camera,
                    RenderMode::Solid,
                    ShaderType::AccretionDisk,
                    &uniforms,
                );
            }
            CentralBody::Star {
                position,
                scale,
                shader,
            } => {
                render_model(
                    &mut framebuffer,
                    &vertex_array,
                    *position,
                    *scale,
                    render::Rotation::Euler(Vector3::zero()),
                    &camera,
                    RenderMode::Solid,
                    *shader,
                    &uniforms,
                );
            }
            CentralBody::Binary {
                separation,
                angle,
                scale,
                shader,
            } => {
                let star1_pos = Vector3::new(
                    angle.cos() * separation / 2.0,
                    0.0,
                    angle.sin() * separation / 2.0,
                );
                let star2_pos = Vector3::new(
                    -angle.cos() * separation / 2.0,
                    0.0,
                    -angle.sin() * separation / 2.0,
                );

                render_model(
                    &mut framebuffer,
                    &vertex_array,
                    star1_pos,
                    *scale,
                    render::Rotation::Euler(Vector3::zero()),
                    &camera,
                    RenderMode::Solid,
                    *shader,
                    &uniforms,
                );

                render_model(
                    &mut framebuffer,
                    &vertex_array,
                    star2_pos,
                    *scale,
                    render::Rotation::Euler(Vector3::zero()),
                    &camera,
                    RenderMode::Solid,
                    ShaderType::WaterWorld, // Different shader for variety
                    &uniforms,
                );
            }
        }

        // Render planets
        for planet in &solar_system.planets {
            let planet_pos = planet.calculate_position(solar_system.center.position());

            render_model(
                &mut framebuffer,
                &vertex_array,
                planet_pos,
                planet.scale,
                render::Rotation::Euler(planet.rotation),
                &camera,
                RenderMode::Solid,
                planet.shader,
                &uniforms,
            );
        }
        // Temporarily change this:
        let render_position = Vector3::new(
            spaceship.position.x,
            spaceship.position.y + 1.0, // Adjust Y offset (try different values)
            spaceship.position.z - 30.0,
        );

        render_model(
            &mut framebuffer,
            &centered_spaceship_vertices,
            spaceship.position, // Use adjusted position
            0.5,                // Try increasing scale: 0.8, 1.0, 1.5
            render::Rotation::Euler(Vector3::zero()),
            &camera,
            RenderMode::Solid,
            ShaderType::Rocky,
            &uniforms,
        );

        /*
        render_model(
            &mut framebuffer,
            &centered_spaceship_vertices, // ← Use the centered version
            spaceship.position,
            spaceship.scale,
            render::Rotation::Matrix(spaceship.get_rotation_matrix()),
            &camera,
            RenderMode::Solid,
            ShaderType::Rocky,
            &uniforms,
        );
        */

        // Draw to screen
        let texture = handle
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .expect("The texture loaded from the color buffer should be valid");

        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            draw_handle.clear_background(Color::WHITE);
            draw_handle.draw_texture(&texture, 0, 0, Color::WHITE);

            draw_handle.draw_text(
                &format!(
                    "Planetas: {} | SPACE=Pausa | P=Agregar | R=Remover",
                    solar_system.planets.len()
                ),
                10,
                10,
                20,
                Color::WHITE,
            );
            draw_handle.draw_text(
                "Centro: B=Agujero Negro | C=Estrella | N=Binario",
                10,
                35,
                20,
                Color::WHITE,
            );

            let status = if paused { "PAUSADO" } else { "ACTIVO" };
            draw_handle.draw_text(status, 10, 60, 20, Color::WHITE);
        }
    }
}

#[inline(always)]
pub fn sample_star(x: i32, y: i32, screen_width: i32, screen_height: i32) -> f32 {
    // Hash screen coordinates to create consistent star positions
    let mut hash = x.wrapping_mul(73856093);
    hash ^= y.wrapping_mul(19349663);
    hash = hash.wrapping_mul(hash);

    // Star density control - 99.7% of pixels are empty space
    if hash % 1000 != 0 {
        return 0.0;
    }

    // Generate brightness for this star (0.3 to 1.0)
    let brightness_hash = hash.wrapping_mul(2147483647);
    let brightness = 0.3 + ((brightness_hash % 700) as f32 / 1000.0);

    // Occasional bright stars (1% chance)
    if (brightness_hash % 100) == 0 {
        brightness * 1.8
    } else {
        brightness
    }
}

/// Render starfield directly to framebuffer
pub fn render_starfield(color_buffer: &mut Image, width: i32, height: i32, star_color: Color) {
    for y in 0..height {
        for x in 0..width {
            let brightness = sample_star(x, y, width, height);

            if brightness > 0.0 {
                let r = (star_color.r as f32 * brightness) as u8;
                let g = (star_color.g as f32 * brightness) as u8;
                let b = (star_color.b as f32 * brightness) as u8;

                color_buffer.draw_pixel(x, y, Color::new(r, g, b, 255));
            }
        }
    }
}
