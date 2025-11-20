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

const NUM_LIGHTS: usize = 3;

const LIGHT_0_POSITION: Vector3 = Vector3 {
    x: 3.0,
    y: 5.0,
    z: 5.0,
};
const LIGHT_0_INTENSITY: f32 = 10.0;
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
const LIGHT_1_INTENSITY: f32 = 0.0;
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
const LIGHT_2_INTENSITY: f32 = 0.0;
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

    let cube_oak_log = Cube::new_with_center(
        Vector3::new(-2.0, 3.0, 0.0),
        CUBE_SIZE,
        [Material::OAK_LOG(Some(texture_ids::OAK_LOG)); 6],
    );

    let cube_diamond = Cube::new_with_center(
        Vector3::new(3.0, 3.0, 0.0),
        CUBE_SIZE,
        [Material::DIAMOND(Some(texture_ids::DIAMONG_BLOCK)); 6],
    );

    // Create a metallic sphere to the left
    let cube_gold = Cube::new_with_center(
        Vector3::new(-3.0, 0.0, 0.0),
        CUBE_SIZE,
        [Material::GOLD(Color::new(180, 180, 200, 255), Some(texture_ids::GOLD)); 6],
    );

    let cube_glass = Cube::new_with_center(
        Vector3::new(0.0, 2.0, 2.0),
        CUBE_SIZE,
        [Material::GLASS(Some(texture_ids::GLASS)); 6],
    );

    let cube_mirror = Cube::new_with_center(
        Vector3::new(0.0, 4.0, -5.0),
        CUBE_SIZE,
        [Material::MIRROR(Some(texture_ids::GLASS)); 6],
    );
    let cube_glowstone = Cube::new_with_center(
        Vector3::new(7.0, 3.0, 0.0),
        CUBE_SIZE,
        [Material::GLOWSTONE(Some(texture_ids::GLOWSTONE)); 6],
    );

    let cube_stone = Cube::new_with_center(
        Vector3::new(0.0, 2.0, -7.0),
        CUBE_SIZE,
        [Material::STONE(Some(texture_ids::STONE)); 6],
    );

    let cube_grass = Cube::new_with_center(
        Vector3::new(5.0, 5.0, 5.0),
        CUBE_SIZE,
        [
            Material::DIRT(Some(texture_ids::DIRT)),
            Material::DIRT(Some(texture_ids::DIRT)),
            Material::DIRT(Some(texture_ids::DIRT)),
            Material::DIRT(Some(texture_ids::GRASS_TOP)),
            Material::DIRT(Some(texture_ids::DIRT)),
            Material::DIRT(Some(texture_ids::DIRT)),
        ],
    );

    let cube_oak_leaves = Cube::new_with_center(
        Vector3::new(0.0, 2.0, -5.0),
        CUBE_SIZE,
        [Material::LEAVES(Some(texture_ids::OAK_LEAVES)); 6],
    );

    let cube_emerald_ore = Cube::new_with_center(
        Vector3::new(0.0, 5.0, -7.0),
        CUBE_SIZE,
        [Material::EMERALD_ORE(Some(texture_ids::EMERALD_ORE)); 6],
    );

    let cube_iron_ore = Cube::new_with_center(
        Vector3::new(4.0, 2.0, -7.0),
        CUBE_SIZE,
        [Material::IRON_ORE(Some(texture_ids::IRON_ORE)); 6],
    );

    let cube_honeycomb = Cube::new_with_center(
        Vector3::new(2.0, 2.0, -7.0),
        CUBE_SIZE,
        [Material::HONEYCOMB(Some(texture_ids::HONEYCOMB)); 6],
    );
    let cube_ice = Cube::new_with_center(
        Vector3::new(2.0, 5.0, -7.0),
        CUBE_SIZE,
        [Material::ICE(Some(texture_ids::ICE)); 6],
    );

    let cube_stripped_oak_log = Cube::new_with_center(
        Vector3::new(-2.0, 5.0, -7.0),
        CUBE_SIZE,
        [Material::STRIPPED_OAK_LOG(Some(texture_ids::STRIPPED_OAK_LOG)); 6],
    );

    let cube_tnt = Cube::new_with_center(
        Vector3::new(-2.0, -5.0, -7.0),
        CUBE_SIZE,
        [Material::TNT(Some(texture_ids::TNT)); 6],
    );

    /*let cube_emissive = Cube::new_with_center(
        Vector3::new(-4.0, 0.5, 2.0),
        0.8,
        [Material::EMISSIVE(Color::new(0, 255, 100, 255), 100.0); 6],
    );*/

    let objects = vec![
        //Object::Cube(cube),
        Object::Cube(cube_gold),
        Object::Cube(cube_glass),
        //        Object::Cube(cube_emissive),
        Object::Cube(cube_oak_log),
        Object::Cube(cube_diamond),
        Object::Cube(cube_grass),
        Object::Cube(cube_glowstone),
        Object::Cube(cube_stone),
        Object::Cube(cube_oak_leaves),
        Object::Cube(cube_emerald_ore),
        Object::Cube(cube_iron_ore),
        Object::Cube(cube_honeycomb),
        Object::Cube(cube_ice),
        Object::Cube(cube_stripped_oak_log),
        Object::Cube(cube_tnt),
        Object::Cube(cube_mirror),
    ];

    // NEW: Build BVH from objects
    println!("Building BVH...");
    let bvh = BVH::build(objects);
    println!("BVH built successfully!");

    let mut lights = vec![
        Light::soft(LIGHT_0_POSITION, LIGHT_0_INTENSITY, LIGHT_0_COLOR, 0.8),
        Light::soft(LIGHT_1_POSITION, LIGHT_1_INTENSITY, LIGHT_1_COLOR, 0.5),
        Light::soft(LIGHT_2_POSITION, LIGHT_2_INTENSITY, LIGHT_2_COLOR, 0.6),
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
    println!("P: Pause/Resume time");
    println!("T/G: Speed up/slow down time");
    println!("R: Reverse time");
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

        if handle.is_key_pressed(KeyboardKey::KEY_N) {
            texture_manager.next_pack();
        }

        // NEW: Pass BVH to render instead of objects
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
