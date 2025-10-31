//maze.rs
use raylib::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Maze = Vec<Vec<char>>;

/// Loads a maze from a text file and returns the maze and player starting position
pub fn load_maze(filename: &str, block_size: usize) -> (Maze, Vector2) {
    let file = File::open(filename).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut maze: Maze = Vec::new();
    let mut player_pos = None;

    for (row_index, line) in reader.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (col_index, ch) in line.expect("Error reading line").chars().enumerate() {
            if ch == 'p' {
                let x = (col_index * block_size + block_size / 2) as f32;
                let y = (row_index * block_size + block_size / 2) as f32;
                player_pos = Some(Vector2 { x, y });
                row.push(' '); // Player position becomes walkable space
            } else {
                row.push(ch);
            }
        }
        maze.push(row);
    }

    let position = player_pos.expect("No player position ('p') found in maze");
    (maze, position)
}

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
