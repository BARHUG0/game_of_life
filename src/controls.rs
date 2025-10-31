//controls.rs
use raylib::prelude::*;

use crate::command::PlayerCommand;

/// Process keyboard input and return a list of commands
/// This function only translates input to commands, doesn't modify game state
pub fn process_input(window: &RaylibHandle) -> Vec<PlayerCommand> {
    let mut commands = Vec::new();

    // Rotation
    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        commands.push(PlayerCommand::rotate_left());
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        commands.push(PlayerCommand::rotate_right());
    }

    // Movement - using WASD controls
    // W = forward, S = backward, A = strafe left, D = strafe right
    if window.is_key_down(KeyboardKey::KEY_W) {
        commands.push(PlayerCommand::move_forward());
    }
    if window.is_key_down(KeyboardKey::KEY_S) {
        commands.push(PlayerCommand::move_backward());
    }
    if window.is_key_down(KeyboardKey::KEY_A) {
        commands.push(PlayerCommand::strafe_left());
    }
    if window.is_key_down(KeyboardKey::KEY_D) {
        commands.push(PlayerCommand::strafe_right());
    }

    commands
}
