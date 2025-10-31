//main.rs
#![allow(warnings)]

mod command;
mod controls;
mod fog_of_war;
mod framebuffer;
mod maze;
mod maze_generator;
mod player;
mod ray;
mod raycaster;
mod renderer;

use rand::Rng;
use raylib::prelude::*;

use std::time::Duration;
use std::{f32::consts::PI, thread};

use command::PlayerCommand;
use controls::process_input;
use fog_of_war::FogOfWar;
use framebuffer::Framebuffer;
use maze_generator::generate_large_maze;
use player::Player;
use raycaster::{cast_rays, cast_single_ray};
use renderer::Renderer;

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

    let vision_radius = block_size as f32 * 4.0; // Player can see 4 cells away
    let mut fog_of_war = FogOfWar::new(&maze, block_size, vision_radius);

    let renderer = Renderer::new(block_size);

    let minimap_size = 200;
    let minimap_x = WINDOW_WIDTH - minimap_size - 20;
    let minimap_y = 20;

    while !&handle.window_should_close() {
        let commands = process_input(&handle);
        for command in commands {
            player.execute_command(command, &maze, block_size);
        }

        renderer.render_maze(&mut framebuffer, &maze);
        renderer.render_player(&mut framebuffer, &player);
        // For multiple rays (full FOV)
        let fov = PI / 3.0; // 60 degrees
        let num_rays = 320; // One ray per column (for 3D view later)
        let rays = cast_rays(&player, &maze, block_size, fov, num_rays);
        // Update fog of war with line-of-sight
        fog_of_war.update(&player, &rays, &maze);

        renderer.render_debug_rays(&mut framebuffer, &player, &rays);
        renderer.render_minimap(
            &mut framebuffer,
            &maze,
            &fog_of_war,
            &player,
            minimap_x,
            minimap_y,
            minimap_size,
        );

        let texture = handle
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .expect("The texture loaded from the color buffer should be valid");

        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            draw_handle.draw_texture(&texture, 0, 0, Color::LIGHTGRAY);

            //draw_handle.gui_button(rectangle, "Hello Word");
        }
    }
}
