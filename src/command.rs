//command.rs
use std::f32::consts::PI;

/// Commands that can be executed by the player or AI entities
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerCommand {
    MoveForward(f32),  // distance to move forward
    MoveBackward(f32), // distance to move backward
    StrafeLeft(f32),   // distance to strafe left
    StrafeRight(f32),  // distance to strafe right
    RotateLeft(f32),   // angle in radians
    RotateRight(f32),  // angle in radians
}

impl PlayerCommand {
    // Constants for default movement values
    pub const MOVE_SPEED: f32 = 7.0;
    pub const ROTATION_SPEED: f32 = PI / 33.0;

    /// Create a forward movement command with default speed
    pub fn move_forward() -> Self {
        PlayerCommand::MoveForward(Self::MOVE_SPEED)
    }

    /// Create a backward movement command with default speed
    pub fn move_backward() -> Self {
        PlayerCommand::MoveBackward(Self::MOVE_SPEED)
    }

    /// Create a strafe left command with default speed
    pub fn strafe_left() -> Self {
        PlayerCommand::StrafeLeft(Self::MOVE_SPEED)
    }

    /// Create a strafe right command with default speed
    pub fn strafe_right() -> Self {
        PlayerCommand::StrafeRight(Self::MOVE_SPEED)
    }

    /// Create a rotate left command with default speed
    pub fn rotate_left() -> Self {
        PlayerCommand::RotateLeft(Self::ROTATION_SPEED)
    }

    /// Create a rotate right command with default speed
    pub fn rotate_right() -> Self {
        PlayerCommand::RotateRight(Self::ROTATION_SPEED)
    }
}
