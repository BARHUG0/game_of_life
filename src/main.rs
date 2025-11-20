#![allow(warnings)]

mod bvh; // NEW: Add BVH module
mod camera;
mod day_night_cycle;
mod framebuffer;
mod intersection;
mod light;
mod material;
mod objects;
mod renderer;
mod skybox;

mod texture;
mod texture_pack;

// Add this for the texture pack config
mod texture_packs {
    pub mod optimum_realism;
}

use bvh::BVH; // NEW: Import BVH
use camera::Camera;
use day_night_cycle::DayNightCycle;
use framebuffer::Framebuffer;
use light::Light;
use material::Material;
use objects::{Cube, Object, Sphere};
use raylib::prelude::*;
use renderer::render;
use skybox::{GradientSkybox, Skybox, SolidSkybox};
use texture_pack::TextureManager;
use texture_packs::optimum_realism::{load_default_pack, texture_ids};

const WINDOW_WIDTH: i32 = 1900;
const WINDOW_HEIGHT: i32 = 1000;

const FRAMEBUFFER_WIDTH: i32 = WINDOW_WIDTH;
const FRAMEBUFFER_HEIGHT: i32 = WINDOW_HEIGHT;

const CYCLE_DURATION: f32 = 60.0;
const STARTING_TIME: f32 = 0.45;

const BLOOM_THRESHOLD: f32 = 0.5;
const BLOOM_RADIUS: i32 = 2;
const BLOOM_INTENSITY: f32 = 0.8;

const CUBE_SIZE: f32 = 1.0;

fn main() {
    game_loop();
}

fn game_loop() {
    let (mut handle, raylib_thread) = raylib::init()
        .undecorated()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Raytracer - BVH Accelerated")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT, Color::WHITE);

    framebuffer.set_background_color(Color::new(80, 80, 200, 255));

    // Initialize day/night cycle
    let mut day_night = DayNightCycle::starting_at_noon(CYCLE_DURATION);

    // Update skybox with new time-of-day values
    let (zenith, horizon, ground) = day_night.get_sky_colors();
    let sun_dir = day_night.get_sun_direction();
    let (sun_color, sun_intensity, sun_size) = day_night.get_sun_properties();
    let moon_dir = day_night.get_moon_direction();
    let (moon_color, moon_intensity, moon_size) = day_night.get_moon_properties();
    let (halo_size, halo_intensity) = day_night.get_halo_properties();
    let stars_intensity = day_night.get_star_intensity();

    let mut skybox = Skybox::Gradient(GradientSkybox::from_cycle(
        zenith,
        horizon,
        ground,
        sun_dir,
        sun_color,
        sun_size,
        sun_intensity,
        moon_dir,
        moon_color,
        moon_size,
        moon_intensity,
        halo_size,
        halo_intensity,
        stars_intensity,
    ));

    let mut texture_manager = TextureManager::new();

    // Load default pack
    let default_pack = load_default_pack();
    texture_manager.add_pack(default_pack);

    let (objects, mut lights) = create_crystal_greenhouse();

    let bvh = BVH::build(objects);

    // Update camera starting position for better view:
    let mut camera = Camera::new(
        Vector3::new(-8.0, 6.0, -8.0), // Position camera at an angle
        Vector3::new(0.0, 1.0, 0.0),   // Look at center of scene
        Vector3::new(0.0, 1.0, 0.0),
    );

    let rotation_speed = PI as f32 / 100.0;
    let camera_move_speed = 0.1; // Adjust for desired movement speed

    println!("=== Controls ===");
    println!("Arrow Keys: Rotate camera around center");
    println!("WASD: Move camera (W=forward, S=back, A=left, D=right)");
    println!("Q/E: Move camera (Q=down, E=up)");
    println!("P: Pause/Resume time");
    println!("T/G: Speed up/slow down time");
    println!("R: Reverse time");
    println!("================");

    while !&handle.window_should_close() {
        framebuffer.clear();

        // Get delta time for smooth animation
        let delta_time = handle.get_frame_time();

        // Day/Night cycle controls
        if handle.is_key_pressed(KeyboardKey::KEY_P) {
            day_night.toggle_pause();
            println!(
                "Time {}",
                if day_night.is_paused() {
                    "PAUSED"
                } else {
                    "RUNNING"
                }
            );
        }
        if handle.is_key_pressed(KeyboardKey::KEY_T) {
            day_night.speed_up();
            println!("Time speed: {:.2}x", day_night.time_speed());
        }
        if handle.is_key_pressed(KeyboardKey::KEY_G) {
            day_night.slow_down();
            println!("Time speed: {:.2}x", day_night.time_speed());
        }
        if handle.is_key_pressed(KeyboardKey::KEY_R) {
            day_night.reverse();
            println!(
                "Time direction reversed! Speed: {:.2}x",
                day_night.time_speed()
            );
        }

        // Update day/night cycle
        day_night.update(delta_time);

        // Update skybox with new time-of-day values
        let (zenith, horizon, ground) = day_night.get_sky_colors();
        let sun_dir = day_night.get_sun_direction();
        let (sun_color, sun_intensity, sun_size) = day_night.get_sun_properties();
        let moon_dir = day_night.get_moon_direction();
        let (moon_color, moon_intensity, moon_size) = day_night.get_moon_properties();
        let (halo_size, halo_intensity) = day_night.get_halo_properties();
        let stars_intensity = day_night.get_star_intensity();

        skybox = Skybox::Gradient(GradientSkybox::from_cycle(
            zenith,
            horizon,
            ground,
            sun_dir,
            sun_color,
            sun_size,
            sun_intensity,
            moon_dir,
            moon_color,
            moon_size,
            moon_intensity,
            halo_size,
            halo_intensity,
            stars_intensity,
        ));

        // Camera rotation controls (Arrow keys)
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

        // Camera translation controls (WASD + Q/E)
        let mut forward = 0.0;
        let mut right = 0.0;
        let mut up = 0.0;

        if handle.is_key_down(KeyboardKey::KEY_W) {
            forward += camera_move_speed;
        }
        if handle.is_key_down(KeyboardKey::KEY_S) {
            forward -= camera_move_speed;
        }
        if handle.is_key_down(KeyboardKey::KEY_D) {
            right += camera_move_speed;
        }
        if handle.is_key_down(KeyboardKey::KEY_A) {
            right -= camera_move_speed;
        }
        if handle.is_key_down(KeyboardKey::KEY_E) {
            up += camera_move_speed;
        }
        if handle.is_key_down(KeyboardKey::KEY_Q) {
            up -= camera_move_speed;
        }

        // Apply camera movement
        if forward != 0.0 || right != 0.0 || up != 0.0 {
            camera.translate(forward, right, up);
        }

        if handle.is_key_pressed(KeyboardKey::KEY_N) {
            texture_manager.next_pack();
        }

        // Render with BVH
        render(
            &mut framebuffer,
            &bvh,
            &camera,
            &lights,
            &skybox,
            &texture_manager,
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

pub fn create_glowstone_shrine() -> (Vec<Object>, Vec<Light>) {
    let mut objects = Vec::new();

    // Center offset for this diorama (positioned at origin)
    let offset_x = 0.0;
    let offset_z = 0.0;

    // === FLOOR LAYER (5x5 grass/dirt base) ===
    for x in -2..=2 {
        for z in -2..=2 {
            // Create a stone path cross pattern leading to center
            let is_path = (x == 0 || z == 0) && !(x == 0 && z == 0);

            let top_material = if is_path {
                Material::STONE(Some(texture_ids::STONE))
            } else {
                Material::GRASS_TOP(Some(texture_ids::GRASS_TOP))
            };

            objects.push(Object::Cube(Cube::new_with_center(
                Vector3::new((x as f32) + offset_x, -1.0, (z as f32) + offset_z),
                CUBE_SIZE,
                [
                    Material::DIRT(Some(texture_ids::DIRT)), // Bottom
                    Material::DIRT(Some(texture_ids::DIRT)), // Front
                    Material::DIRT(Some(texture_ids::DIRT)), // Back
                    top_material,                            // Top
                    Material::DIRT(Some(texture_ids::DIRT)), // Left
                    Material::DIRT(Some(texture_ids::DIRT)), // Right
                ],
            )));
        }
    }

    // === CENTRAL GLOWSTONE SHRINE (2x2 elevated platform) ===
    // Base layer of the shrine (stone pedestal)
    for x in -1..=0 {
        for z in -1..=0 {
            objects.push(Object::Cube(Cube::new_with_center(
                Vector3::new(
                    (x as f32) + 0.5 + offset_x,
                    0.0,
                    (z as f32) + 0.5 + offset_z,
                ),
                CUBE_SIZE,
                [Material::STONE(Some(texture_ids::STONE)); 6],
            )));
        }
    }

    // Glowstone top (2x2, the star of the show!)
    for x in -1..=0 {
        for z in -1..=0 {
            objects.push(Object::Cube(Cube::new_with_center(
                Vector3::new(
                    (x as f32) + 0.5 + offset_x,
                    1.0,
                    (z as f32) + 0.5 + offset_z,
                ),
                CUBE_SIZE,
                [Material::GLOWSTONE(Some(texture_ids::GLOWSTONE)); 6],
            )));
        }
    }

    // === CORNER PILLARS (oak log with leaves caps) ===
    let pillar_positions = [
        (-2.0, -2.0), // Front-left
        (2.0, -2.0),  // Front-right
        (-2.0, 2.0),  // Back-left
        (2.0, 2.0),   // Back-right
    ];

    for (px, pz) in pillar_positions.iter() {
        // Oak log pillar (2 blocks tall)
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new(px + offset_x, 0.0, pz + offset_z),
            CUBE_SIZE,
            [Material::OAK_LOG(Some(texture_ids::OAK_LOG)); 6],
        )));
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new(px + offset_x, 1.0, pz + offset_z),
            CUBE_SIZE,
            [Material::OAK_LOG(Some(texture_ids::OAK_LOG)); 6],
        )));

        // Leaves cap on top
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new(px + offset_x, 2.0, pz + offset_z),
            CUBE_SIZE,
            [Material::LEAVES(Some(texture_ids::OAK_LEAVES)); 6],
        )));
    }

    // === DECORATIVE ACCENTS ===
    // Honeycomb blocks flanking the shrine (left and right)
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(-2.0 + offset_x, 0.0, 0.0 + offset_z),
        CUBE_SIZE,
        [Material::HONEYCOMB(Some(texture_ids::HONEYCOMB)); 6],
    )));
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(2.0 + offset_x, 0.0, 0.0 + offset_z),
        CUBE_SIZE,
        [Material::HONEYCOMB(Some(texture_ids::HONEYCOMB)); 6],
    )));

    // Emerald ore accent blocks (front corners of path)
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(-1.0 + offset_x, 0.0, -2.0 + offset_z),
        CUBE_SIZE,
        [Material::EMERALD_ORE(Some(texture_ids::EMERALD_ORE)); 6],
    )));
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(1.0 + offset_x, 0.0, -2.0 + offset_z),
        CUBE_SIZE,
        [Material::EMERALD_ORE(Some(texture_ids::EMERALD_ORE)); 6],
    )));

    // Single ice block to catch the warm glow (back of shrine)
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(0.0 + offset_x, 0.0, 2.0 + offset_z),
        CUBE_SIZE,
        [Material::ICE(Some(texture_ids::ICE)); 6],
    )));
    let lights = vec![
        // Soft ambient light from above
        /*
                Light::soft(
                    Vector3::new(0.0 + offset_x, 8.0, 0.0 + offset_z),
                    15.0,
                    Color::new(200, 220, 255, 255), // Cool ambient
                    0.6,
                ),
        */
        // Subtle side rim lights for depth (four cardinal directions)
        Light::soft(
            Vector3::new(-6.0 + offset_x, 3.0, 0.0 + offset_z),
            4.0,
            Color::new(180, 200, 255, 255), // Cool blue rim from left
            0.3,
        ),
        Light::soft(
            Vector3::new(6.0 + offset_x, 3.0, 0.0 + offset_z),
            4.0,
            Color::new(255, 230, 200, 255), // Warm orange rim from right
            0.25,
        ),
        Light::soft(
            Vector3::new(0.0 + offset_x, 3.0, -6.0 + offset_z),
            4.0,
            Color::new(255, 200, 220, 255), // Soft pink rim from front
            0.28,
        ),
        Light::soft(
            Vector3::new(0.0 + offset_x, 3.0, 6.0 + offset_z),
            4.0,
            Color::new(200, 255, 220, 255), // Soft green rim from back
            0.26,
        ),
    ];

    (objects, lights)
}

pub fn create_crystal_greenhouse() -> (Vec<Object>, Vec<Light>) {
    let mut objects = Vec::new();

    // Center offset for this diorama (position as needed)
    let offset_x = 0.0;
    let offset_z = 0.0;

    // === FLOOR LAYER (5x5 mixed surface) ===
    for x in -2..=2 as i32 {
        for z in -2..=2 as i32 {
            // Interior has grass/dirt, exterior has stone border
            let is_border = x.abs() == 2 || z.abs() == 2;

            let top_material = if is_border {
                Material::STONE(Some(texture_ids::STONE))
            } else {
                Material::GRASS_TOP(Some(texture_ids::GRASS_TOP))
            };

            objects.push(Object::Cube(Cube::new_with_center(
                Vector3::new((x as f32) + offset_x, -1.0, (z as f32) + offset_z),
                CUBE_SIZE,
                [
                    Material::DIRT(Some(texture_ids::DIRT)), // Bottom
                    Material::DIRT(Some(texture_ids::DIRT)), // Front
                    Material::DIRT(Some(texture_ids::DIRT)), // Back
                    top_material,                            // Top
                    Material::DIRT(Some(texture_ids::DIRT)), // Left
                    Material::DIRT(Some(texture_ids::DIRT)), // Right
                ],
            )));
        }
    }

    // === FRONT GLASS WALL (complete 5-wide wall) ===
    for x in -1..=1 {
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new((x as f32) + offset_x, 2.0, -2.0 + offset_z),
            CUBE_SIZE,
            [Material::GLASS(Some(texture_ids::GLASS)); 6],
        )));
    }

    // Add a second layer for extra glass effect
    for x in -1..=1 {
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new((x as f32) + offset_x, 1.0, -2.0 + offset_z),
            CUBE_SIZE,
            [Material::GLASS(Some(texture_ids::GLASS)); 6],
        )));
    }

    // === INTERIOR TREE (oak log trunk with extending leaves) ===
    // Tree trunk (4 blocks tall, centered in back area)
    for y in 0..2 {
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new(0.0 + offset_x, y as f32, 1.0 + offset_z),
            CUBE_SIZE,
            [Material::OAK_LOG(Some(texture_ids::OAK_LOG)); 6],
        )));
    }

    // Tree canopy - larger crown that extends beyond 5x5 boundaries
    // Layer 1 (y=3): Wide base layer
    for x in -2..=2 as i32 {
        for z in -1..=3 as i32 {
            // Skip the trunk center
            if x == 0 && z == 1 {
                continue;
            }
            // Create rounded shape
            if x.abs() + (z - 1).abs() <= 2 {
                objects.push(Object::Cube(Cube::new_with_center(
                    Vector3::new(x as f32 + offset_x, 2.0, z as f32 + offset_z),
                    CUBE_SIZE,
                    [Material::LEAVES(Some(texture_ids::OAK_LEAVES)); 6],
                )));
            }
        }
    }

    // Layer 2 (y=4): Middle layer
    for x in -1..=1 as i32 {
        for z in 0..=2 as i32 {
            if x.abs() + (z - 1).abs() <= 1 {
                objects.push(Object::Cube(Cube::new_with_center(
                    Vector3::new(x as f32 + offset_x, 3.0, z as f32 + offset_z),
                    CUBE_SIZE,
                    [Material::LEAVES(Some(texture_ids::OAK_LEAVES)); 6],
                )));
            }
        }
    }

    // Layer 3 (y=5): Top cap
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(0.0 + offset_x, 4.0, 1.0 + offset_z),
        CUBE_SIZE,
        [Material::LEAVES(Some(texture_ids::OAK_LEAVES)); 6],
    )));

    // Honeycomb decorative blocks (sides near tree)
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(-1.0 + offset_x, 0.0, 1.0 + offset_z),
        CUBE_SIZE,
        [Material::HONEYCOMB(Some(texture_ids::HONEYCOMB)); 6],
    )));
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(1.0 + offset_x, 0.0, 1.0 + offset_z),
        CUBE_SIZE,
        [Material::HONEYCOMB(Some(texture_ids::HONEYCOMB)); 6],
    )));

    // TNT decorative crate (side)
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(-2.0 + offset_x, 0.0, 0.0 + offset_z),
        CUBE_SIZE,
        [Material::TNT(Some(texture_ids::TNT)); 6],
    )));

    // === LIGHTS ===
    let lights = vec![
        Light::soft(
            Vector3::new(0.0 + offset_x, 15.0, 0.0 + offset_z),
            18.0,
            Color::new(255, 250, 240, 255), // Warm sunlight
            0.7,
        ),
        Light::soft(
            Vector3::new(0.0 + offset_x, 1.0, -4.0 + offset_z),
            5.0,
            Color::new(200, 230, 255, 255), // Cool backlight
            0.35,
        ),
        Light::soft(
            Vector3::new(-5.0 + offset_x, 2.0, 0.0 + offset_z),
            3.5,
            Color::new(200, 220, 255, 255), // Cool blue from left
            0.25,
        ),
        Light::soft(
            Vector3::new(5.0 + offset_x, 2.0, 0.0 + offset_z),
            3.5,
            Color::new(255, 240, 220, 255), // Warm from right
            0.22,
        ),
    ];

    (objects, lights)
}

// Returns: (objects, lights)
pub fn create_treasure_vault() -> (Vec<Object>, Vec<Light>) {
    let mut objects = Vec::new();

    // Center offset for this diorama (position as needed)
    let offset_x = 0.0;
    let offset_z = 0.0;

    // === FLOOR LAYER (5x5 stone floor with checkerboard pattern) ===
    for x in -2..=2 as i32 {
        for z in -2..=2 as i32 {
            // Create a checkerboard pattern in the center area
            let is_center = x.abs() <= 1 && z.abs() <= 1;
            let is_dark = (x + z) % 2 == 0;

            let top_material = if is_center && is_dark {
                Material::GRASS_TOP(Some(texture_ids::GRASS_TOP)) // Dark squares
            } else {
                Material::STONE(Some(texture_ids::STONE)) // Light squares
            };

            objects.push(Object::Cube(Cube::new_with_center(
                Vector3::new((x as f32) + offset_x, -1.0, (z as f32) + offset_z),
                CUBE_SIZE,
                [
                    Material::STONE(Some(texture_ids::STONE)), // Bottom
                    Material::STONE(Some(texture_ids::STONE)), // Front
                    Material::STONE(Some(texture_ids::STONE)), // Back
                    top_material,                              // Top
                    Material::STONE(Some(texture_ids::STONE)), // Left
                    Material::STONE(Some(texture_ids::STONE)), // Right
                ],
            )));
        }
    }

    // === TREASURES ON FLOOR (2x2 center, visible at y=0) ===
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(0.0 + offset_x, 2.0, 2.0 + offset_z),
        CUBE_SIZE,
        [Material::DIAMOND(Some(texture_ids::DIAMONG_BLOCK)); 6],
    )));

    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(1.0 + offset_x, 0.0, -1.0 + offset_z),
        CUBE_SIZE,
        [Material::GOLD(Color::new(255, 215, 0, 255), Some(texture_ids::GOLD)); 6],
    )));
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(-1.0 + offset_x, 0.0, -1.0 + offset_z),
        CUBE_SIZE,
        [Material::GOLD(Color::new(255, 215, 0, 255), Some(texture_ids::GOLD)); 6],
    )));
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(0.0 + offset_x, 0.0, -1.0 + offset_z),
        CUBE_SIZE,
        [Material::GOLD(Color::new(255, 215, 0, 255), Some(texture_ids::GOLD)); 6],
    )));

    // === CAVE ENTRANCE STRUCTURE (symmetric arc from back) ===

    // Ground level (y=0) - Full back wall
    for x in -2..=2 {
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new((x as f32) + offset_x, 0.0, 2.0 + offset_z),
            CUBE_SIZE,
            [Material::STONE(Some(texture_ids::STONE)); 6],
        )));
    }

    // Left side pillar (y=0-4, x=-2)
    for y in 1..=3 {
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new(-2.0 + offset_x, y as f32, 2.0 + offset_z),
            CUBE_SIZE,
            [Material::STONE(Some(texture_ids::STONE)); 6],
        )));
    }

    // Right side pillar (y=0-4, x=2)
    for y in 1..=3 {
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new(2.0 + offset_x, y as f32, 2.0 + offset_z),
            CUBE_SIZE,
            [Material::STONE(Some(texture_ids::STONE)); 6],
        )));
    }

    // Second layer inward (y=1-4, x=±1)
    for y in 1..=2 {
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new(-1.0 + offset_x, y as f32, 2.0 + offset_z),
            CUBE_SIZE,
            [Material::STONE(Some(texture_ids::STONE)); 6],
        )));
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new(1.0 + offset_x, y as f32, 2.0 + offset_z),
            CUBE_SIZE,
            [Material::STONE(Some(texture_ids::STONE)); 6],
        )));
    }

    // Center column (y=1-3, x=0) - shorter to create arch opening
    for y in 1..=1 {
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new(0.0 + offset_x, y as f32, 2.0 + offset_z),
            CUBE_SIZE,
            [Material::STONE(Some(texture_ids::STONE)); 6],
        )));
    }

    // Top of arch (y=4) - spans across creating the arch top

    // Ceiling extending forward (creating cave depth)
    // Layer at z=1 (y=3-4): Wider ceiling
    for x in -2..=2 {
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new((x as f32) + offset_x, 4.0, 1.0 + offset_z),
            CUBE_SIZE,
            [Material::STONE(Some(texture_ids::STONE)); 6],
        )));
    }
    for x in -1..=1 {
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new((x as f32) + offset_x, 3.0, 1.0 + offset_z),
            CUBE_SIZE,
            [Material::STONE(Some(texture_ids::STONE)); 6],
        )));
    }

    // Layer at z=0 (y=4): Middle ceiling
    for x in -1..=1 {
        objects.push(Object::Cube(Cube::new_with_center(
            Vector3::new((x as f32) + offset_x, 4.0, 0.0 + offset_z),
            CUBE_SIZE,
            [Material::STONE(Some(texture_ids::STONE)); 6],
        )));
    }

    // Layer at z=-1 (y=4): Front edge of cave ceiling
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(0.0 + offset_x, 4.0, -1.0 + offset_z),
        CUBE_SIZE,
        [Material::STONE(Some(texture_ids::STONE)); 6],
    )));

    // === MINERAL VEINS (medium density, mixed through cave structure) ===

    // Emerald ore veins
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(-1.0 + offset_x, 2.0, 1.9 + offset_z),
        CUBE_SIZE,
        [Material::EMERALD_ORE(Some(texture_ids::EMERALD_ORE)); 6],
    )));
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(2.0 + offset_x, 0.0, 1.9 + offset_z),
        CUBE_SIZE,
        [Material::EMERALD_ORE(Some(texture_ids::EMERALD_ORE)); 6],
    )));
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(-2.0 + offset_x, 3.0, 1.9 + offset_z),
        CUBE_SIZE,
        [Material::EMERALD_ORE(Some(texture_ids::EMERALD_ORE)); 6],
    )));

    // Iron ore veins
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(2.0 + offset_x, 2.0, 1.90 + offset_z),
        CUBE_SIZE,
        [Material::IRON_ORE(Some(texture_ids::IRON_ORE)); 6],
    )));
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(-2.0 + offset_x, 0.0, 1.90 + offset_z),
        CUBE_SIZE,
        [Material::IRON_ORE(Some(texture_ids::IRON_ORE)); 6],
    )));
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(0.0 + offset_x, 1.0, 1.90 + offset_z),
        CUBE_SIZE,
        [Material::IRON_ORE(Some(texture_ids::IRON_ORE)); 6],
    )));

    // === TNT ON TOP OF CAVE ARC ===
    objects.push(Object::Cube(Cube::new_with_center(
        Vector3::new(0.0 + offset_x, 3.0, -0.0 + offset_z),
        CUBE_SIZE,
        [Material::TNT(Some(texture_ids::TNT)); 6],
    )));

    // === LIGHTS ===
    let lights = vec![
        // Low fill light (simulating reflected light from treasures)
        Light::soft(
            Vector3::new(0.0 + offset_x, 3.0, -5.0 + offset_z),
            5.0,
            Color::new(255, 235, 200, 255), // Warm golden bounce
            0.35,
        ),
        // Subtle side rim lights for depth (four cardinal directions)
        Light::soft(
            Vector3::new(-4.5 + offset_x, 2.5, 0.0 + offset_z),
            4.0,
            Color::new(180, 200, 255, 255), // Cool blue from left
            0.28,
        ),
        Light::soft(
            Vector3::new(4.5 + offset_x, 2.5, 0.0 + offset_z),
            4.0,
            Color::new(255, 230, 200, 255), // Warm orange from right
            0.26,
        ),
        Light::soft(
            Vector3::new(0.0 + offset_x, 2.5, -4.5 + offset_z),
            4.0,
            Color::new(255, 220, 240, 255), // Soft pink from front
            0.27,
        ),
    ];

    (objects, lights)
}
