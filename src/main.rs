//main.rs
#![allow(warnings)]

mod framebuffer;
mod maze;
mod player;
mod raycaster;

use rand::Rng;
use raylib::prelude::*;

use std::time::Duration;
use std::{f32::consts::PI, thread};

use framebuffer::Framebuffer;
use maze::{Maze, load_maze, render_maze, render_player};
use player::Player;
use raycaster::cast_ray;

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
    let (maze, player_pos) = load_maze("assets/maze.txt", block_size);
    let mut player = Player::new(player_pos, PI / 3.0);

    while !&handle.window_should_close() {
        render_maze(&mut framebuffer, &maze, block_size);
        render_player(&mut framebuffer, player.position());
        cast_ray(&mut framebuffer, &maze, &player, block_size);

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

fn process_events(handle: &RaylibHandle, player: &mut Player) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0;

    if handle.is_key_down(KeyboardKey::KEY_S) {}
}
