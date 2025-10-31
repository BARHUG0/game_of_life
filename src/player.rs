//player.rs
use raylib::prelude::*;

use crate::maze::Maze;

pub struct Player {
    position: Vector2,
    angle_of_view: f32,
}

impl Player {
    pub fn new(position: Vector2, angle_of_view: f32) -> Self {
        Player {
            position,
            angle_of_view,
        }
    }

    // Getters
    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn angle_of_view(&self) -> f32 {
        self.angle_of_view
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }

    pub fn y(&self) -> f32 {
        self.position.y
    }

    // Setters
    pub fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle_of_view = angle;
    }

    // Movement methods
    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn rotate(&mut self, delta: f32) {
        self.angle_of_view += delta;
    }

    pub fn try_move(&mut self, dx: f32, dy: f32, maze: &Maze, block_size: usize) {
        let new_x = self.position.x + dx;
        let new_y = self.position.y + dy;

        let i = (new_y / block_size as f32) as usize;
        let j = (new_x / block_size as f32) as usize;

        if i >= maze.len() || j >= maze[0].len() {
            return;
        }

        let cell = maze[i][j];
        if cell == ' ' {
            self.position.x = new_x;
            self.position.y = new_y;
        }
    }

    /// Execute a player command with collision detection
    pub fn execute_command(
        &mut self,
        command: crate::command::PlayerCommand,
        maze: &Maze,
        block_size: usize,
    ) {
        use crate::command::PlayerCommand;

        match command {
            PlayerCommand::MoveForward(speed) => {
                let dir_x = self.angle_of_view.cos();
                let dir_y = self.angle_of_view.sin();
                self.try_move(dir_x * speed, dir_y * speed, maze, block_size);
            }
            PlayerCommand::MoveBackward(speed) => {
                let dir_x = self.angle_of_view.cos();
                let dir_y = self.angle_of_view.sin();
                self.try_move(-dir_x * speed, -dir_y * speed, maze, block_size);
            }
            PlayerCommand::StrafeLeft(speed) => {
                let dir_x = self.angle_of_view.cos();
                let dir_y = self.angle_of_view.sin();
                // Perpendicular to viewing direction (rotate 90 degrees left)
                self.try_move(dir_y * speed, -dir_x * speed, maze, block_size);
            }
            PlayerCommand::StrafeRight(speed) => {
                let dir_x = self.angle_of_view.cos();
                let dir_y = self.angle_of_view.sin();
                // Perpendicular to viewing direction (rotate 90 degrees right)
                self.try_move(-dir_y * speed, dir_x * speed, maze, block_size);
            }
            PlayerCommand::RotateLeft(angle) => {
                self.rotate(-angle);
            }
            PlayerCommand::RotateRight(angle) => {
                self.rotate(angle);
            }
        }
    }
}
