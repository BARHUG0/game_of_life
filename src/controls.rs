use rand::rand_core::block;
use raylib::prelude::*;
use std::f32::consts::PI;

use crate::{maze::Maze, player::Player};

pub fn process_input(window: &RaylibHandle, player: &mut Player, maze: &Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 0.5;
    const ROTATION_SPEED: f32 = PI / 60.0;

    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.rotate(-ROTATION_SPEED);
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.rotate(ROTATION_SPEED);
    }

    let dir_x = player.angle_of_view().cos();
    let dir_y = player.angle_of_view().sin();

    if window.is_key_down(KeyboardKey::KEY_D) {
        player.try_move(dir_x * MOVE_SPEED, dir_y * MOVE_SPEED, maze, block_size);
    }
    if window.is_key_down(KeyboardKey::KEY_A) {
        player.try_move(-dir_x * MOVE_SPEED, -dir_y * MOVE_SPEED, maze, block_size);
    }
    if window.is_key_down(KeyboardKey::KEY_W) {
        player.try_move(dir_y * MOVE_SPEED, -dir_x * MOVE_SPEED, maze, block_size);
    }
    if window.is_key_down(KeyboardKey::KEY_S) {
        player.try_move(-dir_y * MOVE_SPEED, dir_x * MOVE_SPEED, maze, block_size);
    }
}
