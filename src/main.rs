//main.rs
#![allow(warnings)]

mod command;
mod controls;
mod fog_of_war;
mod framebuffer;
mod game_state;
mod maze;
mod maze_generator;
mod player;
mod ray;
mod raycaster;
mod renderer;
mod sprite;
mod sprite_renderer;
mod ui_renderer;
mod wall_renderer;

use rand::Rng;
use raylib::prelude::*;

use std::time::Duration;
use std::{f32::consts::PI, thread};

use command::PlayerCommand;
use controls::process_input;
use fog_of_war::FogOfWar;
use framebuffer::Framebuffer;
use game_state::GameState;
use maze_generator::generate_large_maze;
use player::Player;
use raycaster::{cast_rays, cast_single_ray};
use renderer::Renderer;
use sprite::{process_pickups, spawn_sprites_in_maze};
use sprite_renderer::SpriteRenderer;
use ui_renderer::UIRenderer;
use wall_renderer::WallRenderer;

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

    let mut rng = rand::rng();

    let block_size = 64;
    let (maze, player_pos) = generate_large_maze(block_size);
    let mut player = Player::new(player_pos, 0.0);

    // Game state for tracking pickups
    let mut game_state = GameState::new();
    let mut pickup_message = String::new();
    let mut message_timer = 0.0;

    // Spawn more sprites now that the algorithm is improved
    let num_sprites = 25;
    let mut sprites = spawn_sprites_in_maze(&maze, block_size, num_sprites);

    let vision_radius = block_size as f32 * 4.0;
    let mut fog_of_war = FogOfWar::new(&maze, block_size, vision_radius);

    let renderer = Renderer::new(block_size);

    // Load wall textures
    let textures = load_wolfenstein_textures(&mut handle, &raylib_thread);
    let wall_renderer = WallRenderer::new(WINDOW_WIDTH, WINDOW_HEIGHT, textures);

    // Load sprite textures
    let sprite_textures = load_sprite_textures(&mut handle, &raylib_thread);
    let sprite_renderer = SpriteRenderer::new(WINDOW_WIDTH, WINDOW_HEIGHT, sprite_textures);

    let ui_renderer = UIRenderer::new(WINDOW_WIDTH, WINDOW_HEIGHT, 100);

    let minimap_size = 200;
    let minimap_x = WINDOW_WIDTH - minimap_size - 20;
    let minimap_y = 20;

    while !&handle.window_should_close() {
        let delta_time = handle.get_frame_time();

        let commands = process_input(&handle);
        for command in commands {
            player.execute_command(command, &maze, block_size);
        }

        // Process pickups
        let collected = process_pickups(&mut sprites, player.x(), player.y());
        for pickup in collected {
            let message = game_state.collect_pickup(pickup);
            pickup_message = message;
            message_timer = 2.0; // Show message for 2 seconds
        }

        // Update message timer
        if message_timer > 0.0 {
            message_timer -= delta_time;
        }

        framebuffer.clear();

        // Render 3D view
        wall_renderer.render_floor_ceiling(&mut framebuffer);

        // Cast rays for raycasting
        let fov = PI / 3.0; // 60 degrees
        let num_rays = 320;
        let rays = cast_rays(&player, &maze, block_size, fov, num_rays);

        // Render walls
        wall_renderer.render_3d_view(&mut framebuffer, &rays, &player, block_size);

        // Render sprites (AFTER walls, with depth testing)
        sprite_renderer.render_sprites(&mut framebuffer, &sprites, &player, &rays, block_size);

        // Update fog of war
        fog_of_war.update(&player, &rays, &maze);

        // Render minimap
        renderer.render_minimap(
            &mut framebuffer,
            &maze,
            &fog_of_war,
            &player,
            minimap_x,
            minimap_y,
            minimap_size,
        );

        // Render sprites on minimap
        let cell_size = minimap_size / maze[0].len().max(maze.len()) as i32;
        sprite_renderer.render_sprites_minimap(
            &mut framebuffer,
            &sprites,
            minimap_x,
            minimap_y,
            cell_size,
            block_size,
        );

        ui_renderer.render_hud(&mut framebuffer, &game_state);

        let texture = handle
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .expect("The texture loaded from the color buffer should be valid");

        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            draw_handle.draw_texture(&texture, 0, 0, Color::LIGHTGRAY);

            // Draw UI elements
            draw_handle.draw_text(&game_state.format_stats(), 10, 10, 20, Color::WHITE);

            // Draw pickup message if active
            if message_timer > 0.0 {
                let msg_x = WINDOW_WIDTH / 2 - 100;
                let msg_y = WINDOW_HEIGHT - 100;
                draw_handle.draw_text(&pickup_message, msg_x, msg_y, 30, Color::YELLOW);
            }
        }
    }
}

fn load_wolfenstein_textures(handle: &mut RaylibHandle, thread: &RaylibThread) -> Vec<Image> {
    vec![
        Image::load_image("assets/level1_default_bright.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco1_bright.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco2_bright.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco3_bright.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_default_dark.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco1_dark.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco2_dark.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco3_dark.png").expect("Failed to load texture"),
    ]
}

fn load_sprite_textures(handle: &mut RaylibHandle, thread: &RaylibThread) -> Vec<Image> {
    vec![
        // Decorations (0-3)
        Image::load_image("assets/sprite_barrel.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::BROWN)),
        Image::load_image("assets/sprite_pillar.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::GRAY)),
        Image::load_image("assets/sprite_lamp.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::YELLOW)),
        Image::load_image("assets/sprite_plant.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::GREEN)),
        // Pickups (4-7)
        Image::load_image("assets/pickup_health.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::RED)),
        Image::load_image("assets/pickup_ammo.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::ORANGE)),
        Image::load_image("assets/pickup_key.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::GOLD)),
        Image::load_image("assets/pickup_treasure.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::SKYBLUE)),
    ]
}
