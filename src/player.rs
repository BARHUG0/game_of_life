//player.rs
use raylib::prelude::*;
use std::f32::consts::PI;

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

    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn angle_of_view(&self) -> f32 {
        self.angle_of_view
    }
}

impl Player {
    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn rotate(&mut self, delta: f32) {
        self.angle_of_view += delta;
    }
}

impl Player {
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
}
