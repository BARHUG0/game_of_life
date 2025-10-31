//controls.rs
use raylib::prelude::*;

use crate::command::PlayerCommand;

/// Input state for the current frame
#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub movement_commands: Vec<PlayerCommand>,
    pub is_shooting: bool,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            movement_commands: Vec::new(),
            is_shooting: false,
        }
    }
}

/// Process keyboard and mouse input and return input state
pub fn process_input(window: &RaylibHandle) -> InputState {
    let mut input_state = InputState::new();

    // Rotation
    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        input_state
            .movement_commands
            .push(PlayerCommand::rotate_left());
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        input_state
            .movement_commands
            .push(PlayerCommand::rotate_right());
    }

    // Movement - using WASD controls
    // W = forward, S = backward, A = strafe left, D = strafe right
    if window.is_key_down(KeyboardKey::KEY_W) {
        input_state
            .movement_commands
            .push(PlayerCommand::move_forward());
    }
    if window.is_key_down(KeyboardKey::KEY_S) {
        input_state
            .movement_commands
            .push(PlayerCommand::move_backward());
    }
    if window.is_key_down(KeyboardKey::KEY_A) {
        input_state
            .movement_commands
            .push(PlayerCommand::strafe_left());
    }
    if window.is_key_down(KeyboardKey::KEY_D) {
        input_state
            .movement_commands
            .push(PlayerCommand::strafe_right());
    }

    // Shooting - left mouse button
    input_state.is_shooting = window.is_key_down(KeyboardKey::KEY_SPACE);

    input_state
}
