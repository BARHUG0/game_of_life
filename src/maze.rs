//maze.rs
use raylib::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Maze = Vec<Vec<char>>;

/// Helper function to check if a position is valid and walkable
pub fn is_walkable(maze: &Maze, x: f32, y: f32, block_size: usize) -> bool {
    let i = (y / block_size as f32) as usize;
    let j = (x / block_size as f32) as usize;

    if i >= maze.len() || j >= maze[0].len() {
        return false;
    }

    maze[i][j] == ' '
}

/// Helper function to get cell at world coordinates
pub fn get_cell(maze: &Maze, x: f32, y: f32, block_size: usize) -> Option<char> {
    let i = (y / block_size as f32) as usize;
    let j = (x / block_size as f32) as usize;

    if i >= maze.len() || j >= maze[0].len() {
        return None;
    }

    Some(maze[i][j])
}

/// Helper function to get cell at grid coordinates
pub fn get_cell_at_grid(maze: &Maze, grid_x: usize, grid_y: usize) -> Option<char> {
    if grid_y >= maze.len() || grid_x >= maze[0].len() {
        return None;
    }

    Some(maze[grid_y][grid_x])
}
