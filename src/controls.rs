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

    // Keyboard rotation
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

    // Mouse rotation
    let mouse_delta = window.get_mouse_delta();
    let mouse_sensitivity = 0.003; // Adjust this for faster/slower rotation

    if mouse_delta.x.abs() > 0.1 {
        let rotation_amount = mouse_delta.x * mouse_sensitivity;
        if rotation_amount > 0.0 {
            input_state
                .movement_commands
                .push(PlayerCommand::RotateRight(rotation_amount));
        } else {
            input_state
                .movement_commands
                .push(PlayerCommand::RotateLeft(-rotation_amount));
        }
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

    // Shooting - left mouse button or space
    input_state.is_shooting = window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
        || window.is_key_down(KeyboardKey::KEY_SPACE);

    input_state
}
